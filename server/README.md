# opaque-wasm server
[![License](https://img.shields.io/github/license/squirrelchat/opaque-wasm.svg?style=flat-square)](https://github.com/squirrelchat/opaque-wasm/blob/mistress/LICENSE)
[![npm](https://img.shields.io/npm/v/@squirrelchat/opaque-wasm-server?label=npm%20%28server%29&style=flat-square)](https://npm.im/@squirrelchat/opaque-wasm-server)

## Installation
This package is only compatible with Node at this time.

```
npm i @squirrelchat/opaque-wasm-server
yarn add @squirrelchat/opaque-wasm-server
pnpm add @squirrelchat/opaque-wasm-server
```

## Usage
### Startup
```js
import { Server } from '@squirrelchat/opaque-wasm-server'

// Create a server
// -> First time
const server = new Server()
console.log(server.getState()) // <Uint8Array ...>
// ~> Save this somewhere super safe!

// -> Future times
const state = ... // Get from secure storage
const server = new Server(state)

// The state MUST be stored and restored, otherwise
// logging in will not work after a server restart.
```

### Registration
```js
const username = ... // <~ value sent by the client
const request = ... // <~ value sent by the client

try {
	const response = server.startRegistration(username, request)
	console.log(response) // <Uint8Array ...>
	// ~> send this to the client
} catch (e) {
	console.error('Could not start registration!', e)
}

// ---

const username = ... // <~ value sent by the client
const record = ... // <~ value sent by the client

try {
	const credentials = server.finishRegistration(record)
	console.log(credentials) // <Uint8Array ...>
	// Store this on disk, and tada!
} catch (e) {
	console.error('Could not finalize registration!', e)
}
```

### Login
```js
const username = ... // <~ value sent by the client
const request = ... // <~ value sent by the client

// A note on account enumeration:
// The OPAQUE protocol protects against account enumeration
// by design during authentication. To achieve this, you must
// engage in the protocol even if the account does not exists.
// opaque-wasm and the underlying lib does this by using a fake
// random record when no record is specified.
try {
	const { response, state } = server.startLogin(username, request, row?.credentials)

	console.log(state) // <Uint8Array ...>
	// Store this somewhere SAFE, you'll need it to finalize the login.
	// Do NOT send it to the client!

	console.log(response) // <Uint8Array ...>
	// ~> send this to the client
} catch (e) {
	console.error('Could not start authentication!', e)
}

// ---

const authentication = ... // <~ value sent by the client
try {
	const sessionKey = server.finishLogin(state, authentication)
	console.log('session key:', sessionKey)
} catch (e) {
	console.error('Could not finalize authentication!', e)
}
```
