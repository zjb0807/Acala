// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

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

//! Autogenerated weights for nutsfinance_stable_asset
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-21-58`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for nutsfinance_stable_asset.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> nutsfinance_stable_asset::WeightInfo for WeightInfo<T> {
	// Storage: `StableAsset::PoolCount` (r:1 w:1)
	// Proof: `StableAsset::PoolCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1345`
		//  Estimated: `4810`
		// Minimum execution time: 23_512 nanoseconds.
		Weight::from_parts(24_404_000, 4810)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn modify_a() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1538`
		//  Estimated: `5003`
		// Minimum execution time: 19_083 nanoseconds.
		Weight::from_parts(19_777_000, 5003)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn modify_fees() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1538`
		//  Estimated: `5003`
		// Minimum execution time: 18_622 nanoseconds.
		Weight::from_parts(19_252_000, 5003)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn modify_recipients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1538`
		//  Estimated: `5003`
		// Minimum execution time: 18_790 nanoseconds.
		Weight::from_parts(19_288_000, 5003)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:10 w:10)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `u` is `[2, 5]`.
	fn mint(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2295 + u * (186 ±0)`
		//  Estimated: `6196 + u * (5244 ±0)`
		// Minimum execution time: 146_869 nanoseconds.
		Weight::from_parts(75_425_004, 6196)
			// Standard Error: 339_226
			.saturating_add(Weight::from_parts(39_908_669, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 5244).saturating_mul(u.into()))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:6 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `u` is `[2, 5]`.
	fn swap(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2522 + u * (183 ±0)`
		//  Estimated: `6005 + u * (2622 ±32)`
		// Minimum execution time: 1_944_725 nanoseconds.
		Weight::from_parts(523_486_450, 6005)
			// Standard Error: 5_239_232
			.saturating_add(Weight::from_parts(766_234_558, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_parts(0, 2622).saturating_mul(u.into()))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:10 w:10)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_proportion(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2464 + u * (248 ±0)`
		//  Estimated: `5947 + u * (5244 ±0)`
		// Minimum execution time: 172_984 nanoseconds.
		Weight::from_parts(105_866_691, 5947)
			// Standard Error: 239_021
			.saturating_add(Weight::from_parts(37_629_465, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 5244).saturating_mul(u.into()))
	}
	// Storage: `StableAsset::Pools` (r:1 w:0)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:4 w:0)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_single(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1592 + u * (244 ±0)`
		//  Estimated: `5116 + u * (1016 ±3)`
		// Minimum execution time: 1_516_064 nanoseconds.
		Weight::from_parts(766_696_141, 5116)
			// Standard Error: 1_406_657
			.saturating_add(Weight::from_parts(391_543_413, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1016).saturating_mul(u.into()))
	}
	// Storage: `StableAsset::Pools` (r:1 w:1)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:10 w:10)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_multi(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2464 + u * (248 ±0)`
		//  Estimated: `5947 + u * (5244 ±32)`
		// Minimum execution time: 149_017 nanoseconds.
		Weight::from_parts(66_062_356, 5947)
			// Standard Error: 457_261
			.saturating_add(Weight::from_parts(46_701_446, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 5244).saturating_mul(u.into()))
	}
}
