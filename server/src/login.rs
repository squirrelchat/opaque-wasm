// Copyright (c) Squirrel Chat et al., All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its contributors
//    may be used to endorse or promote products derived from this software without
//    specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::server::Server;
use alloc::vec::Vec;
use js_sys::{Object, Reflect, Uint8Array};
use opaque_ke::{
	CredentialFinalization, CredentialRequest, ServerLogin, ServerLoginStartParameters,
	ServerRegistration,
};
use opaque_wasm_core::OpaqueWasmCipherSuite;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Server {
	#[wasm_bindgen(js_name = "startLogin")]
	pub fn start_login(
		&mut self,
		identifier: &str,
		request: &[u8],
		record: Option<Vec<u8>>,
	) -> Result<Object, JsValue> {
		let message = CredentialRequest::deserialize(request)
			.or::<JsValue>(Err("could not deserialize login request".into()))?;

		let registration_record = match record {
			Some(bytes) => Some(
				ServerRegistration::<OpaqueWasmCipherSuite>::deserialize(&bytes)
					.or::<JsValue>(Err("could not deserialize record".into()))?,
			),
			None => None,
		};

		let start_result = ServerLogin::start(
			&mut self.rng,
			&self.internal,
			registration_record,
			message,
			identifier.as_bytes(),
			ServerLoginStartParameters::default(),
		);

		match start_result {
			Ok(result) => {
				let message_buffer = Uint8Array::new(&320.into());
				let state_buffer = Uint8Array::new(&192.into());

				message_buffer.copy_from(result.message.serialize().as_slice());
				state_buffer.copy_from(result.state.serialize().as_slice());

				let object = Object::new();
				Reflect::set(&object, &"message".into(), &message_buffer).unwrap();
				Reflect::set(&object, &"state".into(), &state_buffer).unwrap();

				Ok(object)
			}
			Err(_) => Err("could not start login".into()),
		}
	}

	#[wasm_bindgen(js_name = "finishLogin")]
	pub fn finish_login(&self, state: &[u8], finish: &[u8]) -> Result<Vec<u8>, JsValue> {
		let login = ServerLogin::<OpaqueWasmCipherSuite>::deserialize(state)
			.or::<JsValue>(Err("could not deserialize state".into()))?;

		let message = CredentialFinalization::deserialize(finish)
			.or::<JsValue>(Err("could not deserialize login finish".into()))?;

		let result = login
			.finish(message)
			.or::<JsValue>(Err("could not finalize login".into()))?;

		Ok(result.session_key.to_vec())
	}
}
