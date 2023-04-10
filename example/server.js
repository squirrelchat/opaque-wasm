/**
 * Copyright (c) Squirrel Chat et al.
 * SPDX-License-Identifier: 0BSD
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
 * REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
 * INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
 * LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

import { Server } from '@squirrelchat/opaque-wasm-server'
import { createServer } from 'http'
import { existsSync, createReadStream } from 'fs'
import { readFile, writeFile } from 'fs/promises'
import { join, normalize } from 'path'
import sqlite3 from 'sqlite3'

// ---
// Utility functions, useful for later.
async function readRequest (req) {
  return new Promise((resolve) => {
    let data = ''
    req.on('data', (d) => data += d)
    req.on('end', () => resolve(JSON.parse(data)))
  })
}

// ---
// OPAQUE server creation
// The OPAQUE server holds a keypair and an OPRF seed, that
// must be always the same for logins to work. To create
// these, you can create a server without parameters, and then
// save the state somewhere. This state can then be fetched and
// passed to the server the next time it's created.
//
// Here, the state is saved to a file on disk which is not ideal.
// Up to you to use a better, more secure method!
const hasConfig = existsSync('./opaque-state.bin')
const server = hasConfig
  ? new Server(await readFile('./opaque-state.bin'))
  : new Server()

// Save the config if it just has been created.
if (!hasConfig) await writeFile('./opaque-state.bin', server.getState())

// ---
// For the database, we'll use a simple sqlite database for storing
// users, and a simple in-memory `Map` for holding temporary data.
const memoryStore = new Map()
const db = new sqlite3.Database('./db.sqlite3')
db.run('CREATE TABLE IF NOT EXISTS users (username TEXT NOT NULL, credentials BLOB NOT NULL)')

// ---
// Endpoint implementations

// POST /register/init
async function registrationInit (req, res) {
  const { username, request } = await readRequest(req)
  // Query the database to check if an account already exists.
  db.get('SELECT rowid FROM users WHERE username = ?', [ username ], ((err, row) => {
    if (err) { // Handle a possible SQLite error.
      res.writeHead(500).end(err.toString())
      return
    }

    if (row) { // If an account already exists, abort the request.
      // A note on account enumeration:
      // While this is out of the scope of the OPAQUE protocol, and
      // therefore this lib, it's possible to protect against enumeration
      // during registration using application-level tricks to not give the
      // attacker any clue about whether an account exists or not.
      //
      // You can give a look to this issue on the OPAQUE's Internet Draft
      // GitHub repository: https://github.com/cfrg/draft-irtf-cfrg-opaque/issues/388
      res.writeHead(400).end(JSON.stringify({ error: 'account already exists' }))
      return
    }

    try {
      // Start the registration exchange.
      const response = server.startRegistration(username, request)
      res.end(JSON.stringify({ response: Array.from(response) }))
    } catch (e) {
      // Registration failed! The user probably sent bad data.
      res.writeHead(400).end(JSON.stringify({ error: 'bad request' }))
    }
  }))
}

// POST /register/finalize
async function registrationFinalize (req, res) {
  const { username, record } = await readRequest(req)
  // Query the database to check if an account already exists.
  db.get('SELECT rowid FROM users WHERE username = ?', [ username ], ((err, row) => {
    if (err) { // Handle a possible SQLite error.
      res.writeHead(500).end(err.toString())
      return
    }

    if (row) { // If an account already exists, abort the request.
      res.writeHead(400).end(JSON.stringify({ error: 'account already exists' }))
      return
    }

    try {
      // Finalize the registration, get the credentials blob,
      // and store it in the database.
      const credentials = server.finishRegistration(record)
      db.run('INSERT INTO users (username, credentials) VALUES (?, ?)', [ username, credentials ], (err) => {
        if (err) { // Handle a possible SQLite error
          res.writeHead(500).end(err.toString())
          return
        }

        // Done!
        res.end(JSON.stringify({ success: 'account successfully registered' }))
      })
    } catch (e) {
      // Registration failed! The user probably sent bad data.
      res.writeHead(400).end(JSON.stringify({ error: 'bad request' }))
    }
  }))
}

// POST /login/init
async function loginInit (req, res) {
  const { username, request } = await readRequest(req)
  // Fetch the credentials blob from the database.
  db.get('SELECT credentials FROM users WHERE username = ?', [ username ], (err, row) => {
    if (err) { // Handle a possible SQLite error.
      res.writeHead(500).end(err.toString())
      return
    }

    try {
      // A note on account enumeration:
      // The OPAQUE protocol protects against account enumeration
      // by design during authentication. To achieve this, you must
      // engage in the protocol even if the account does not exists.
      // opaque-wasm and the underlying lib does this by using a fake
      // random record when no record is specified.
      //
      // Whether to do this step or not is up to you. If account
      // enumeration during authentication is not a concern for you,
      // you may skip this and simply send a clear error right now to
      // the client.
      const { response, state } = server.startLogin(username, request, row?.credentials)

      // Store the state in memory for later use.
      // Up to you here to use a better method to hold the state than this!
      // Be careful, the state MUST NOT be seen by the client. Don't shove it
      // in plain in a JSON Web Token for example!!
      memoryStore.set(username, state)

      res.end(JSON.stringify({ response: Array.from(response) }))
    } catch (e) {
      // Login failed! The user probably sent bad data.
      // The reply is a generic bad credentials message,
      // as part of the account-enumeration protection.
      res.writeHead(400).end(JSON.stringify({ error: 'bad request' }))
    }
  })
}

// POST /login/finalize
async function loginFinalize (req, res) {
  const { username, authentication } = await readRequest(req)

  // Get back the state from the in-memory store
  // If there is no state, we abort the request.
  const state = memoryStore.get(username)
  if (!memoryStore.delete(username)) {
    res.writeHead(400).end(JSON.stringify({ error: 'invalid session' }))
    return
  }

  try {
    // Finalize login to get the session key.
    // The session key is known by the server and the client,
    // and can be used a a symmetric key for AES encryption for
    // example. Up to you do to what you want with it!
    //
    // Be aware that each execution of the protocol will yield
    // a different session key.
    const sessionKey = server.finishLogin(state, authentication)

    // Done!
    res.end(JSON.stringify({ success: `welcome back ${username}` }))
  } catch (e) {
    // Login failed! The user probably sent bad data.
    // We reply with a generic bad credentials message,
    // as part of the account-enumeration protection.
    res.writeHead(400).end(JSON.stringify({ error: 'bad credentials' }))
  }
}

// ---
// Server creation and startup!
const httpServer = createServer((req, res) => {
  if (req.method === 'GET') {
    if (req.url === '/') {
      createReadStream('./dist/index.html').pipe(res)
      return
    }

    if (req.url.startsWith('/assets/')) {
      const file = join('dist', normalize(req.url))
      if (existsSync(file)) {
        if (file.endsWith('.css')) res.setHeader('content-type', 'text/css')
        if (file.endsWith('.js')) res.setHeader('content-type', 'text/javascript')
        if (file.endsWith('.wasm')) res.setHeader('content-type', 'application/wasm')
        createReadStream(join('dist', req.url)).pipe(res)
        return
      }
    }
  }

  if (req.method === 'POST') {
    if (req.url === '/registration/init') {
      registrationInit(req, res)
      return
    }

    if (req.url === '/registration/finalize') {
      registrationFinalize(req, res)
      return
    }

    if (req.url === '/login/init') {
      loginInit(req, res)
      return
    }

    if (req.url === '/login/finalize') {
      loginFinalize(req, res)
      return
    }
  }

  res.writeHead(404).end('not found');
})

const port = Number(process.env.PORT) || 1337
httpServer.listen(port, 'localhost')
console.log(`Listening on http://localhost:${port}`)
