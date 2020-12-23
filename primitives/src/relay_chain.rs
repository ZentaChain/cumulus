// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

pub use polkadot_core_primitives::*;
pub use polkadot_primitives::v1;

pub mod well_known_keys {
	use polkadot_primitives::v1::Id as ParaId;
	use hex_literal::hex;
	use sp_io::hashing::twox_64;
	use sp_std::prelude::*;

	pub const ACTIVE_CONFIG: &[u8] =
		&hex!["06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385"];

	pub fn relay_dispatch_queue_size(para_id: ParaId) -> Vec<u8> {
		use codec::Encode as _;

		let prefix = hex!["f5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e"];

		para_id.using_encoded(|para_id: &[u8]| {
			prefix.as_ref()
				.iter()
				.chain(twox_64(para_id).iter())
				.chain(para_id.iter())
				.cloned()
				.collect()
		})
	}
}
