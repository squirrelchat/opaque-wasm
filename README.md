# opaque-wasm
Wrapper for [opaque-ke](https://github.com/facebook/opaque-ke) to implement the OPAQUE protocol in JavaScript/WASM.

This repo contains 2 packages: a client package and a server package. This allows an application to use this library
in a browser without shipping useless server code (and vice-versa). It also allows the use of different compilation
profiles: the client is optimized for size (-Os) and uses [lol_alloc](https://github.com/Craig-Macomber/lol_alloc) as
the allocator, while the server binary is optimized for performance (-O3) and uses the default Rust allocator.

This library uses the following OPAQUE configuration, based on the recommendations of the OPAQUE draft and the Argon2 RFC:
  - OPRF: ristretto255-SHA512
  - KDF: HKDF-SHA-512
  - MAC: HMAC-SHA-512
  - Hash: SHA-512
  - KSF: Argon2id(S = zeroes(16), p = 4, T = Nh, m = 2^16, t = 3, v = 0x13, K = nil, X = nil, y = 2)
  - Group: ristretto255

If you use different implementations for the client and server side, make sure to match the configurations accordingly.

## Installation
> **Warning**: The packages have not been published yet.

### Client
The client requires the use of a bundler compatible with WebAssembly ESM at this time.
(e.g: [Vite](https://vitejs.dev/) with [`vite-plugin-wasm`](https://github.com/Menci/vite-plugin-wasm)).

```
npm i @squirrelchat/opaque-wasm-client
yarn add @squirrelchat/opaque-wasm-client
pnpm add @squirrelchat/opaque-wasm-client
```

### Server
The server is only compatible with Node at this time.

```
npm i @squirrelchat/opaque-wasm-server
yarn add @squirrelchat/opaque-wasm-server
pnpm add @squirrelchat/opaque-wasm-server
```

## Usage
A complete example is available in the [`examples`](examples) folder.

### Client
```js
import { startRegistration, startLogin } from '@squirrelchat/opaque-wasm-client'

// REGISTRATION
const registration = startRegistration('my sup€r sekure passw0rd! uwu')
const response = sendToServerAndGetResponse(registration.request)

const { exportKey, serverPublicKey, record } = registration.finish(response)
sendRegistrationToServer(record)

console.log('export key:', exportKey)
console.log('server public key:', serverPublicKey)

// LOGIN
const login = startLogin('my sup€r sekure passw0rd! uwu')
const response = sendToServerAndGetResponse(login.request)

const { exportKey, sessionKey, serverPublicKey, message } = registration.finish(response)
sendAuthenticationToServer(message)

console.log('export key:', exportKey)
console.log('session key:', sessionKey)
console.log('server public key:', serverPublicKey)
```

### Server
```js
import { Server } from '@squirrelchat/opaque-wasm-server'

// Create a server
// -> First time
const server = new Server()
saveStateSomewhereSave(server.getState())
// -> Future times
const server = new Server(getSavedState())

// The state MUST be stored and restored, otherwise
// logging in will not work after a server restart.

// REGISTRATION
const { username, request } = receiveRequestFromClient()
const response = server.startRegistration(username, request)

const record = sendResponseToClientAndGetRecord(response)
const credentials = server.finishRegistration(record)
saveCredentialsInDatabase(username, credentials)

// LOGIN
const { username, request } = receiveRequestFromClient()
const { response, state } = server.startLogin(username, request, row?.credentials)

const authentication = sendResponseToClientAndGetFinalMessage(response)
const sessionKey = server.finishLogin(state, authentication)

console.log('session key:', sessionKey)
```

## Credits where due
This wrapper is inspired from prior work at [marucjmar/opaque-wasm](https://github.com/marucjmar/opaque-wasm). The
implementation differs in multiple ways, especially regarding the split client/server packages. The API is also very
different.
