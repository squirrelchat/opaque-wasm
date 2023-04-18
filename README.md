# opaque-wasm
[![License](https://img.shields.io/github/license/squirrelchat/opaque-wasm.svg?style=flat-square)](https://github.com/squirrelchat/opaque-wasm/blob/mistress/LICENSE)
[![npm (client)](https://img.shields.io/npm/v/@squirrelchat/opaque-wasm-client?label=npm%20%28client%29&style=flat-square)](https://npm.im/@squirrelchat/opaque-wasm-client)
[![npm (server)](https://img.shields.io/npm/v/@squirrelchat/opaque-wasm-server?label=npm%20%28server%29&style=flat-square)](https://npm.im/@squirrelchat/opaque-wasm-server)

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

## Installation and usage
See the [`client`](client) and [`server`](server) folders for more information about the bits you need.

A complete example (client + server) is available in the [`example`](example) folder, with a lot of comments to guide
you through.

## Credits where due
This wrapper is inspired from prior work at [marucjmar/opaque-wasm](https://github.com/marucjmar/opaque-wasm). The
implementation differs in multiple ways, especially regarding the split client/server packages. The API is also very
different.
