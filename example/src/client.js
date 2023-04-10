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

import './style.css'
import { startRegistration, startLogin } from '@squirrelchat/opaque-wasm-client'

// ---
// Utility functions, useful for later.
function bufferToDump (title, buffer) {
  let str = '\n'
  for (let i = 0; i < buffer.length; i++) {
    str += buffer[i].toString(16).padStart(2, '0')
    str += (i + 1) % 8 ? ' ' : '\n'
  }

  const titleElement = document.createElement('b')
  const textNode = document.createTextNode(str)
  const frag = document.createDocumentFragment()
  frag.appendChild(titleElement)
  frag.appendChild(textNode)

  titleElement.innerText = `${title}:`
  return frag
}

const registerInfoBox = document.querySelector('#section-register .infobox')
const registerBinaryDump = document.querySelector('#section-register .binary')

const loginInfoBox = document.querySelector('#section-login .infobox')
const loginBinaryDump = document.querySelector('#section-login .binary')

// ---
// Registration handling
document.getElementById('form-register').addEventListener('submit', async (evt) => {
  evt.preventDefault() // Prevent the form from actually submitting.
  const username = evt.target.username.value
  const password = evt.target.password.value

  // Clear previous error and debug info.
  Array.from(registerBinaryDump.childNodes).forEach((e) => e.remove())
  registerInfoBox.classList.remove('success')
  registerInfoBox.classList.remove('error')
  registerInfoBox.innerText = ''

  // Verify the user actually put some credentials.
  // You should also verify that the password complies
  // with basic contrains, such as length, complexity, etc!
  if (!username || !password) {
    registerInfoBox.classList.add('error')
    registerInfoBox.innerText = 'You must provide credentials!'
    return
  }

  // Start the registration exchange.
  const register = startRegistration(password)
  const resInit = await fetch('/registration/init', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ username, request: Array.from(register.request) })
  })

  // Retrieve the response from the server.
  const { response, error: initError } = await resInit.json()
  if (initError) {
    // Server reported an error! Abort here.
    registerInfoBox.classList.add('error')
    registerInfoBox.innerText = `init error: ${initError} (HTTP ${resInit.status})`
    return
  }

  // Finalize the registration.
  // We get the server's identity (public key), an export key
  // (that the server does not know), and the record to send
  // to the server to finalize the process.
  let result
  try {
    result = register.finish(response)
  } catch (e) {
    loginInfoBox.classList.add('error')
    loginInfoBox.innerText = e === 'reflected value detected'
      ? 'Protocol violation: server sent a reflected OPRF value.'
      : 'Could not finalize registration'
    return
  }

  const { record, serverPublicKey, exportKey } = result
  const resFinalize = await fetch('/registration/finalize', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ username, record: Array.from(record) })
  })

  // Retrieve response from the server.
  const { success, error } = await resFinalize.json()
  if (error) {
    // Server reported an error! Abort here.
    // Note: in this case, the server public key and
    // the export key are worthless and must be disposed.
    registerInfoBox.classList.add('error')
    registerInfoBox.innerText = `finalize error: ${error} (HTTP ${resFinalize.status})`
    return
  }

  // Success!
  registerInfoBox.classList.add('success')
  registerInfoBox.innerText = success

  // Let's print some stuff on the webpage, for the tester to peek at.
  registerBinaryDump.appendChild(bufferToDump('Export Key', exportKey))
  registerBinaryDump.appendChild(document.createElement('hr'))
  registerBinaryDump.appendChild(bufferToDump('Server Public Key', serverPublicKey))
})

// ---
// Login handling
document.getElementById('form-login').addEventListener('submit', async (evt) => {
  evt.preventDefault() // Prevent the form from actually submitting.
  const username = evt.target.username.value
  const password = evt.target.password.value

  // Clear previous error and debug info.
  Array.from(loginBinaryDump.childNodes).forEach((e) => e.remove())
  loginInfoBox.classList.remove('success')
  loginInfoBox.classList.remove('error')
  loginInfoBox.innerText = ''

  // Verify the user actually put some credentials.
  if (!username || !password) {
    loginInfoBox.classList.add('error')
    loginInfoBox.innerText = 'You must provide credentials!'
  }

  // Start the login exchange.
  const login = startLogin(password)
  const resInit = await fetch('/login/init', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ username, request: Array.from(login.request) })
  })

  // Retrieve response from the server.
  const { response, error: initError } = await resInit.json()
  if (initError) {
    // Server reported an error! Abort here.
    loginInfoBox.classList.add('error')
    loginInfoBox.innerText = `init error: ${initError} (HTTP ${resInit.status})`
    return
  }

  // Finalize the login.
  // We get the server's identity (public key), an export key
  // (that the server does not know), a session key (that the
  // server knows), and the final message to send to the server
  // to finalize the authentication.
  //
  // The server public key can be verified to ensure we're talking
  // to the right server (i.e. the server authenticates itself).
  let result
  try {
    result = login.finish(response)
  } catch (e) {
    loginInfoBox.classList.add('error')
    loginInfoBox.innerText = e === 'reflected value detected'
      ? 'Protocol violation: server sent a reflected OPRF value.'
      : 'Bad credentials'
    return
  }

  const { message, serverPublicKey, sessionKey, exportKey } = result
  const resFinalize = await fetch('/login/finalize', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ username, authentication: Array.from(message) })
  })

  // Retrieve response from the server.
  const { success, error } = await resFinalize.json()
  if (error) {
    // Server reported an error! Abort here.
    // Note: in this case, the session key is worthless
    // as the server does not have it on hand.
    loginInfoBox.classList.add('error')
    loginInfoBox.innerText = `finalize error: ${error} (HTTP ${resFinalize.status})`
    return
  }

  // Success!
  loginInfoBox.classList.add('success')
  loginInfoBox.innerText = success

  // Let's print some stuff on the webpage, for the tester to peek at.
  loginBinaryDump.appendChild(bufferToDump('Export Key', exportKey))
  loginBinaryDump.appendChild(document.createElement('hr'))
  loginBinaryDump.appendChild(bufferToDump('Session Key', sessionKey))
  loginBinaryDump.appendChild(document.createElement('hr'))
  loginBinaryDump.appendChild(bufferToDump('Server Public Key', serverPublicKey))
})
