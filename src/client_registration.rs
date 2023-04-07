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
	errors::ProtocolError, ClientRegistration as OpaqueClientRegistration,
	ClientRegistrationFinishParameters, RegistrationResponse,
};
use rand::rngs::OsRng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClientRegistration {
	rng: OsRng,
	password: Vec<u8>,
	request: Vec<u8>,
	state: OpaqueClientRegistration<DefaultCipher>,
}

#[wasm_bindgen]
impl ClientRegistration {
	#[wasm_bindgen(getter, js_name = "request")]
	pub fn get_message(&self) -> Vec<u8> {
		self.request.clone()
	}

	pub fn finish(mut self, response: &[u8]) -> Result<Object, JsValue> {
		let message = RegistrationResponse::deserialize(response)
			.or::<JsValue>(Err("could not deserialize registration response".into()))?;

		let finish_result = self.state.finish(
			&mut self.rng,
			&self.password[..],
			message,
			ClientRegistrationFinishParameters::default(),
		);

		match finish_result {
			Ok(result) => {
				let export_key = Uint8Array::new(&64.into());
				let server_public_key = Uint8Array::new(&32.into());
				let record = Uint8Array::new(&192.into());

				export_key.copy_from(result.export_key.as_slice());
				server_public_key.copy_from(result.server_s_pk.serialize().as_slice());
				record.copy_from(result.message.serialize().as_slice());

				let object = Object::new();
				Reflect::set(&object, &"exportKey".into(), &export_key).unwrap();
				Reflect::set(&object, &"serverPublicKey".into(), &server_public_key).unwrap();
				Reflect::set(&object, &"record".into(), &record).unwrap();

				Ok(object)
			}
			Err(ProtocolError::ReflectedValueError) => Err("reflected value detected".into()),
			Err(_) => Err("could not finalize registration".into()),
		}
	}
}

#[wasm_bindgen(js_name = "startRegistration")]
pub fn start_registration(password: &str) -> Result<ClientRegistration, JsValue> {
	let mut rng = OsRng;
	let password_bytes = password.as_bytes();
	OpaqueClientRegistration::<DefaultCipher>::start(&mut rng, password_bytes)
		.or(Err("failed to start registration".into()))
		.map(|result| ClientRegistration {
			rng,
			password: password_bytes.to_vec(),
			request: result.message.serialize().to_vec(),
			state: result.state,
		})
}
