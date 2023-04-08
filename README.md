# `@squirrelchat/opaque-wasm`
Wrapper for [opaque-ke](https://github.com/facebook/opaque-ke) to implement the OPAQUE protocol in JavaScript/WASM.

This package ships with 2 WASM binaries: the client binary and the server binary. This allows an application to just
use this library in a browser without shipping useless server code. It also allows the use of different compilation
profiles: the client is optimized for size (-Os) and uses [lol_alloc](https://github.com/Craig-Macomber/lol_alloc) as
the allocator, while the server binary is optimized for performance (-O3) and uses the default Rust allocator.

This library uses the following OPAQUE profile, based on the recommendations of the OPAQUE draft and the Argon2 RFC:
  - ristretto255-SHA512
  - HKDF-SHA-512
  - HMAC-SHA-512
  - SHA-512
  - Argon2id(S = zeroes(16), p = 4, T = Nh, m = 2^16, t = 3, v = 0x13, K = nil, X = nil, y = 2)
  - ristretto255

If you use different implementations for the client and server side, make sure to match the configurations accordingly.

## Installation
TODO

## Usage
TODO

## Credits where due
This wrapper is inspired from prior work at [marucjmar/opaque-wasm](https://github.com/marucjmar/opaque-wasm). The
implementation differs in multiple ways, especially regarding the split client/server binary. The API is also very
different.
