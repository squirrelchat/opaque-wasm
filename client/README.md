# opaque-wasm client
[![License](https://img.shields.io/github/license/squirrelchat/opaque-wasm.svg?style=flat-square)](https://github.com/squirrelchat/opaque-wasm/blob/mistress/LICENSE)
[![npm](https://img.shields.io/npm/v/@squirrelchat/opaque-wasm-client?label=npm%20%28client%29&style=flat-square)](https://npm.im/@squirrelchat/opaque-wasm-client)

Wrapper for [opaque-ke](https://github.com/facebook/opaque-ke) to implement the OPAQUE protocol in JavaScript/WASM.

This library uses the following OPAQUE configuration, based on the recommendations of the OPAQUE draft and the Argon2 RFC:
  - OPRF: ristretto255-SHA512
  - KDF: HKDF-SHA-512
  - MAC: HMAC-SHA-512
  - Hash: SHA-512
  - KSF: Argon2id(S = zeroes(16), p = 4, T = Nh, m = 2^16, t = 3, v = 0x13, K = nil, X = nil, y = 2)
  - Group: ristretto255

## Installation
The client requires an environment compatible with WebAssembly ESM. For example,
 - For the web: [Vite](https://vitejs.dev/) with [`vite-plugin-wasm`](https://github.com/Menci/vite-plugin-wasm)
 - For Node: use the [`--experimental-wasm-modules` flag](https://nodejs.org/api/esm.html#wasm-modules)

```
npm i @squirrelchat/opaque-wasm-client
yarn add @squirrelchat/opaque-wasm-client
pnpm add @squirrelchat/opaque-wasm-client
```

## Usage
### Registration
```js
import { startRegistration } from '@squirrelchat/opaque-wasm-client'

try {
	const registration = startRegistration('mewn supy€w sekyuwe paffw0wdy! UwU')
	console.log(registration.request) // <Uint8Array ...>
	// ~> send this to the server

	const response = ... // <~ response from the server

	const { exportKey, serverPublicKey, record } = registration.finish(response)
	console.log(record) // <Uint8Array ...>
	// ~> send this to the server

	console.log('export key:', exportKey) // <Uint8Array ...>
	console.log('server public key:', serverPublicKey) // <Uint8Array ...>
} catch (e) {
	console.error('Registration failed!', e)
}
```

### Login
```js
import { startLogin } from '@squirrelchat/opaque-wasm-client'

try {
	const login = startLogin('mewn supy€w sekyuwe paffw0wdy! UwU')
	console.log(login.request) // <Uint8Array ...>
	// ~> send this to the server

	const response = ... // <~ response from the server

	const { exportKey, sessionKey, serverPublicKey, message } = registration.finish(response)
	console.log(record) // <Uint8Array ...>
	// ~> send this to the server

	console.log('export key:', exportKey)
	console.log('session key:', sessionKey)
	console.log('server public key:', serverPublicKey)
} catch (e) {
	console.error('Login failed!', e)
}
```
