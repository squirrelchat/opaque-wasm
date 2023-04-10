# opaque-wasm example usage
This folder contains an example usage of the opaque-wasm library to implement a login (and register) form which uses
the OPAQUE protocol.

```sh
pnpm i
pnpm run build
pnpm run start
# Go to http://localhost:1337 in your browser
```

The server and the client are extremely minimal and only implements the required bits for the register and login flow.
A lot of things are out of scope for this example! Here's a non-exhaustive list of things you'll need to figure out by
yourself:
  - [OPAQUE's Application Considerations](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-opaque#name-application-considerations)
  - Authentication cookies
  - Proper credentials identifier handling
  - Password policy validation
    - This step must be done client-side. It defeats the purpose of the protocol to expose the password to the server
      for validating a password policy.
    - There has been some publications about Zero Knowledge Password Policy Checks (ZKPPC), but none is in a usable
    state at this time. In the future, maybe!
      - Zero-Knowledge Password Policy Check from Lattices (2017): https://eprint.iacr.org/2017/854
      - Blind Password Registration for Verifier-based PAKE (2016): https://eprint.iacr.org/2016/442
  - Multi-factor authentication
  - Server scalability
  - Rate limiting
  - ...

The error handling and validation are also very minimal and are absolutely NOT sufficient for a real production app!

## License
Contents of this folder are licensed under the 0BSD license, a public-domain equivalent license.
