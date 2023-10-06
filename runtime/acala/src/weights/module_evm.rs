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

//! Autogenerated weights for module_evm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-42-209`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
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

/// Weight functions for module_evm.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_evm::WeightInfo for WeightInfo<T> {
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1658`
		//  Estimated: `7598`
		// Minimum execution time: 136_282 nanoseconds.
		Weight::from_parts(138_429_000, 7598)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create2() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1658`
		//  Estimated: `7598`
		// Minimum execution time: 131_359 nanoseconds.
		Weight::from_parts(133_155_000, 7598)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::NetworkContractIndex` (r:1 w:1)
	// Proof: `EVM::NetworkContractIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_nft_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1627`
		//  Estimated: `7567`
		// Minimum execution time: 165_557 nanoseconds.
		Weight::from_parts(168_973_000, 7567)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_predeploy_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1575`
		//  Estimated: `7515`
		// Minimum execution time: 169_693 nanoseconds.
		Weight::from_parts(173_166_000, 7515)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::Codes` (r:1 w:0)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2432`
		//  Estimated: `8372`
		// Minimum execution time: 127_483 nanoseconds.
		Weight::from_parts(130_401_000, 8372)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn transfer_maintainer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1842`
		//  Estimated: `5307`
		// Minimum execution time: 90_569 nanoseconds.
		Weight::from_parts(91_608_000, 5307)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn publish_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2313`
		//  Estimated: `5778`
		// Minimum execution time: 128_975 nanoseconds.
		Weight::from_parts(130_202_000, 5778)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn publish_free() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1700`
		//  Estimated: `5165`
		// Minimum execution time: 26_430 nanoseconds.
		Weight::from_parts(27_385_000, 5165)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn enable_contract_development() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1434`
		//  Estimated: `3633`
		// Minimum execution time: 95_911 nanoseconds.
		Weight::from_parts(97_002_000, 3633)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn disable_contract_development() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1522`
		//  Estimated: `3633`
		// Minimum execution time: 97_342 nanoseconds.
		Weight::from_parts(98_516_000, 3633)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:2)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:2)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2448`
		//  Estimated: `8388`
		// Minimum execution time: 154_020 nanoseconds.
		Weight::from_parts(149_799_280, 8388)
			// Standard Error: 15
			.saturating_add(Weight::from_parts(5_610, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:2 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:1 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `IdleScheduler::NextTaskId` (r:1 w:1)
	// Proof: `IdleScheduler::NextTaskId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:0)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `IdleScheduler::Tasks` (r:0 w:1)
	// Proof: `IdleScheduler::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn selfdestruct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2608`
		//  Estimated: `6073`
		// Minimum execution time: 179_759 nanoseconds.
		Weight::from_parts(183_124_000, 6073)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
