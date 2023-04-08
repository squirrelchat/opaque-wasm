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
use alloc::vec::Vec;
use opaque_ke::ServerSetup;
use rand::rngs::OsRng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Server {
	pub(crate) rng: OsRng,
	pub(crate) internal: ServerSetup<DefaultCipher>,
}

#[wasm_bindgen]
impl Server {
	#[wasm_bindgen(constructor)]
	pub fn new(state: Option<Vec<u8>>) -> Result<Server, JsValue> {
		let mut rng = OsRng;

		let setup = match state {
			Some(s) => {
				ServerSetup::deserialize(&s).or::<JsValue>(Err("invalid server state".into()))?
			}
			None => ServerSetup::new(&mut rng),
		};

		Ok(Server {
			rng,
			internal: setup,
		})
	}

	#[wasm_bindgen(js_name = "getState")]
	pub fn get_state(&self) -> Vec<u8> {
		self.internal.serialize().to_vec()
	}
}
