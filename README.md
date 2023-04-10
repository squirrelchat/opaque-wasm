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
### Client
The client requires the use of a bundler compatible with WebAssembly ESM at this time.
(e.g: [Vite](https://vitejs.dev/) with [`vite-plugin-wasm`](https://github.com/Menci/vite-plugin-wasm)).

TODO

### Server
The server is only compatible with Node at this time.

TODO

## Usage
### Client
TODO

### Server
TODO

## Credits where due
This wrapper is inspired from prior work at [marucjmar/opaque-wasm](https://github.com/marucjmar/opaque-wasm). The
implementation differs in multiple ways, especially regarding the split client/server packages. The API is also very
different.
