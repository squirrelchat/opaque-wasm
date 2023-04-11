build: && _fix-typescript
	RUSTFLAGS='-C opt-level=s' wasm-pack build -s squirrelchat --release client
	RUSTFLAGS='-C opt-level=3' wasm-pack build -s squirrelchat --release --target nodejs server

format:
	rustfmt client/**/*.rs core/**/*.rs server/**/*.rs

lint:
	cargo deny check
	cargo clippy
	rustfmt --check client/**/*.rs core/**/*.rs server/**/*.rs

# Replace `object` with proper types, something wasm-bindgen doesn't support because it's a piece of [redacted]
# Oen could use skip_typescript and write all the types manually, but let's just do it this way for now eh.
_fix-typescript:
	#!/usr/bin/env node
	const { readFileSync, writeFileSync } = require('fs')
	let clientTypes = readFileSync('client/pkg/opaque_wasm_client.d.ts', 'utf8')
	let serverTypes = readFileSync('server/pkg/opaque_wasm_server.d.ts', 'utf8')

	clientTypes = clientTypes.replace('object', 'ClientLoginResult').replace('object', 'ClientLoginResult')
	clientTypes = clientTypes.replace('object', 'ClientRegistrationResult').replace('object', 'ClientRegistrationResult')
	serverTypes = serverTypes.replace('object', 'ServerRegistration').replace('object', 'ServerRegistration')

	writeFileSync('client/pkg/opaque_wasm_client.d.ts', clientTypes)
	writeFileSync('server/pkg/opaque_wasm_server.d.ts', serverTypes)
