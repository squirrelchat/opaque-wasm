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
use crate::server::Server;
use opaque_ke::{RegistrationRequest, RegistrationUpload, ServerRegistration};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Server {
	#[wasm_bindgen(js_name = "startRegistration")]
	pub fn start_registration(&self, identifier: &str, request: &[u8]) -> Result<Vec<u8>, JsValue> {
		let message = RegistrationRequest::deserialize(request)
			.or::<JsValue>(Err("could not deserialize registration request".into()))?;

		let result = ServerRegistration::<DefaultCipher>::start(
			&self.internal,
			message,
			identifier.as_bytes(),
		)
		.or::<JsValue>(Err("could not start registration".into()))?;

		Ok(result.message.serialize().to_vec())
	}

	#[wasm_bindgen(js_name = "finishRegistration")]
	pub fn finish_registration(&self, registration_record: &[u8]) -> Result<Vec<u8>, JsValue> {
		let upload = RegistrationUpload::<DefaultCipher>::deserialize(registration_record)
			.or::<JsValue>(Err("could not deserialize upload".into()))?;

		let result = ServerRegistration::<DefaultCipher>::finish(upload);
		Ok(result.serialize().to_vec())
	}
}
