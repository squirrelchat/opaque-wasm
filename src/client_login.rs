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

use crate::cipher::DefaultCipher;
use js_sys::{Object, Reflect, Uint8Array};
use opaque_ke::{
	errors::ProtocolError, ClientLogin as OpaqueClientLogin, ClientLoginFinishParameters,
	CredentialResponse,
};
use rand::rngs::OsRng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClientLogin {
	password: Vec<u8>,
	request: Vec<u8>,
	state: OpaqueClientLogin<DefaultCipher>,
}

#[wasm_bindgen]
impl ClientLogin {
	#[wasm_bindgen(getter, js_name = "request")]
	pub fn get_message(&self) -> Vec<u8> {
		self.request.clone()
	}

	pub fn finish(self, response: &[u8]) -> Result<Object, JsValue> {
		let message = CredentialResponse::deserialize(response)
			.or::<JsValue>(Err("could not deserialize login response".into()))?;

		let finish_result = self.state.finish(
			&self.password[..],
			message,
			ClientLoginFinishParameters::default(),
		);

		match finish_result {
			Ok(result) => {
				let message = Uint8Array::new(&64.into());
				let server_public_key = Uint8Array::new(&32.into());
				let session_key = Uint8Array::new(&64.into());
				let export_key = Uint8Array::new(&64.into());

				message.copy_from(result.message.serialize().as_slice());
				server_public_key.copy_from(result.server_s_pk.serialize().as_slice());
				session_key.copy_from(result.session_key.as_slice());
				export_key.copy_from(result.export_key.as_slice());

				let object = Object::new();
				Reflect::set(&object, &"message".into(), &message).unwrap();
				Reflect::set(&object, &"serverPublicKey".into(), &server_public_key).unwrap();
				Reflect::set(&object, &"sessionKey".into(), &session_key).unwrap();
				Reflect::set(&object, &"exportKey".into(), &export_key).unwrap();

				Ok(object)
			}
			Err(ProtocolError::ReflectedValueError) => Err("reflected value detected".into()),
			Err(_) => Err("could not finalize login".into()),
		}
	}
}

#[wasm_bindgen(js_name = "startLogin")]
pub fn start_login(password: &str) -> Result<ClientLogin, JsValue> {
	let mut rng = OsRng;
	let password_bytes = password.as_bytes();
	OpaqueClientLogin::<DefaultCipher>::start(&mut rng, password_bytes)
		.or(Err("failed to start login".into()))
		.map(|result| ClientLogin {
			password: password_bytes.to_vec(),
			request: result.message.serialize().to_vec(),
			state: result.state,
		})
}
