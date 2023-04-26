build: && _fix-typescript _fix-package-json
	RUSTFLAGS='-C opt-level=s' wasm-pack build -s squirrelchat --release client
	RUSTFLAGS='-C opt-level=3' wasm-pack build -s squirrelchat --release --target nodejs server

format:
	rustfmt client/**/*.rs core/**/*.rs server/**/*.rs

lint:
	cargo deny check
	cargo clippy
	rustfmt --check client/**/*.rs core/**/*.rs server/**/*.rs

publish: build
	cd client/pkg && npm publish --access public
	cd server/pkg && npm publish --access public

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
	console.log('Fixed TypeScript types')

# Have I mentioned how much of a [redacted] [redacted] piece of [redacted] wasm-pack [redacted] is?
# https://github.com/rustwasm/wasm-pack/issues/1193 (Nov 13, 2022)
# https://github.com/rustwasm/wasm-pack/pull/1194 (Nov 13, 2022) - PR still not merged :D
# https://github.com/rustwasm/wasm-pack/pull/1061 (Sep 23, 2021)
# https://github.com/rustwasm/wasm-pack/pull/1089 (Dec 5, 2021)
# At this point I'm close to believing wasm-pack is unmaintained. [clown emoji]
_fix-package-json:
	#!/usr/bin/env node
	const { readFileSync, writeFileSync } = require('fs')
	let clientPackage = readFileSync('client/pkg/package.json', 'utf8')
	let serverPackage = readFileSync('server/pkg/package.json', 'utf8')

	// Add wasm types
	clientPackage = clientPackage.replace('.d.ts"', '.d.ts",\n    "opaque_wasm_client_bg.wasm.d.ts"')
	serverPackage = serverPackage.replace('.d.ts"', '.d.ts",\n    "opaque_wasm_server_bg.wasm.d.ts"')

	// Fix package.json
	clientPackage = clientPackage.replace('"main"', '"type": "module",\n  "main"')
	// serverPackage = serverPackage.replace('"main"', '"type": "module",\n  "main"')

	writeFileSync('client/pkg/package.json', clientPackage)
	writeFileSync('server/pkg/package.json', serverPackage)
	console.log('Fixed package.json `files`')
