// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
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

//! Autogenerated weights for module_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-37-73`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_nft.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_nft::WeightInfo for WeightInfo<T> {
	// Storage: `OrmlNFT::NextClassId` (r:1 w:1)
	// Proof: `OrmlNFT::NextClassId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `Proxy::Proxies` (r:1 w:1)
	// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	// Storage: `OrmlNFT::Classes` (r:0 w:1)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_class() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `6196`
		// Minimum execution time: 91_152 nanoseconds.
		Weight::from_parts(93_056_000, 6196)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `OrmlNFT::Classes` (r:1 w:1)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `OrmlNFT::NextTokenId` (r:1 w:1)
	// Proof: `OrmlNFT::NextTokenId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `OrmlNFT::Tokens` (r:0 w:999)
	// Proof: `OrmlNFT::Tokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `OrmlNFT::TokensByOwner` (r:0 w:999)
	// Proof: `OrmlNFT::TokensByOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[1, 1000]`.
	fn mint(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2496`
		//  Estimated: `6196`
		// Minimum execution time: 110_814 nanoseconds.
		Weight::from_parts(36_375_538, 6196)
			// Standard Error: 18_120
			.saturating_add(Weight::from_parts(23_231_053, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(i.into())))
	}
	// Storage: `OrmlNFT::Classes` (r:1 w:0)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `OrmlNFT::Tokens` (r:1 w:1)
	// Proof: `OrmlNFT::Tokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `OrmlNFT::TokensByOwner` (r:0 w:2)
	// Proof: `OrmlNFT::TokensByOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4845`
		//  Estimated: `8310`
		// Minimum execution time: 112_355 nanoseconds.
		Weight::from_parts(113_167_000, 8310)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: `OrmlNFT::Classes` (r:1 w:1)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `OrmlNFT::Tokens` (r:1 w:1)
	// Proof: `OrmlNFT::Tokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `OrmlNFT::TokensByOwner` (r:0 w:1)
	// Proof: `OrmlNFT::TokensByOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4744`
		//  Estimated: `8209`
		// Minimum execution time: 73_233 nanoseconds.
		Weight::from_parts(74_326_000, 8209)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `OrmlNFT::Classes` (r:1 w:1)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `OrmlNFT::Tokens` (r:1 w:1)
	// Proof: `OrmlNFT::Tokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `OrmlNFT::TokensByOwner` (r:0 w:1)
	// Proof: `OrmlNFT::TokensByOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[0, 3670016]`.
	fn burn_with_remark(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4744`
		//  Estimated: `8209`
		// Minimum execution time: 74_800 nanoseconds.
		Weight::from_parts(75_142_000, 8209)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_669, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `OrmlNFT::Classes` (r:1 w:1)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Proxy::Proxies` (r:1 w:1)
	// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `OrmlNFT::NextTokenId` (r:0 w:1)
	// Proof: `OrmlNFT::NextTokenId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn destroy_class() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2831`
		//  Estimated: `6296`
		// Minimum execution time: 108_230 nanoseconds.
		Weight::from_parts(109_809_000, 6296)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `OrmlNFT::Classes` (r:1 w:1)
	// Proof: `OrmlNFT::Classes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_class_properties() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2339`
		//  Estimated: `5804`
		// Minimum execution time: 19_547 nanoseconds.
		Weight::from_parts(20_101_000, 5804)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
