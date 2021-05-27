// This file is part of Minterest.

// Copyright (C) 2021 Minterest finance.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for minterest_protocol
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-19, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/minterest
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=minterest_protocol
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/minterest-protocol/src/weights.rs
// --template=./templates/weight-template-for-pallet.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for minterest_protocol.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn deposit_underlying() -> Weight;
	fn redeem() -> Weight;
	fn redeem_underlying() -> Weight;
	fn redeem_wrapped() -> Weight;
	fn borrow() -> Weight;
	fn repay() -> Weight;
	fn repay_all() -> Weight;
	fn repay_on_behalf() -> Weight;
	fn transfer_wrapped() -> Weight;
	fn enable_is_collateral() -> Weight;
	fn disable_is_collateral() -> Weight;
	fn claim_mnt() -> Weight;
}

/// Weights for minterest_protocol using the Minterest node and recommended hardware.
pub struct MinterestWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for MinterestWeight<T> {
	fn create_pool() -> Weight {
		(51_388_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn deposit_underlying() -> Weight {
		(372_381_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(17 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn redeem() -> Weight {
		(661_936_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(41 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn redeem_underlying() -> Weight {
		(680_118_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(41 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn redeem_wrapped() -> Weight {
		(657_706_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(41 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn borrow() -> Weight {
		(720_791_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(41 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn repay() -> Weight {
		(403_918_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(16 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn repay_all() -> Weight {
		(408_738_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(16 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn repay_on_behalf() -> Weight {
		(403_362_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(16 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn transfer_wrapped() -> Weight {
		(644_237_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(43 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn enable_is_collateral() -> Weight {
		(62_467_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn disable_is_collateral() -> Weight {
		(334_020_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(31 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn claim_mnt() -> Weight {
		(1_136_790_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(35 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_pool() -> Weight {
		(51_388_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn deposit_underlying() -> Weight {
		(372_381_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(17 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn redeem() -> Weight {
		(661_936_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(41 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn redeem_underlying() -> Weight {
		(680_118_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(41 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn redeem_wrapped() -> Weight {
		(657_706_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(41 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn borrow() -> Weight {
		(720_791_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(41 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn repay() -> Weight {
		(403_918_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(16 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn repay_all() -> Weight {
		(408_738_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(16 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn repay_on_behalf() -> Weight {
		(403_362_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(16 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn transfer_wrapped() -> Weight {
		(644_237_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(43 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn enable_is_collateral() -> Weight {
		(62_467_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn disable_is_collateral() -> Weight {
		(334_020_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(31 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn claim_mnt() -> Weight {
		(1_136_790_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(35 as Weight))
			.saturating_add(RocksDbWeight::get().writes(15 as Weight))
	}
}
