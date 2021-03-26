use crate::{CurrencyId, Operation, Rate, Runtime};

use frame_system::RawOrigin;
use orml_benchmarking::runtime_benchmarks;
use sp_runtime::FixedPointNumber;
use sp_std::prelude::*;

runtime_benchmarks! {
	{ Runtime, controller }

	_ {}

	pause_specific_operation {
	}: _(
		RawOrigin::Root,
		CurrencyId::DOT,
		Operation::Deposit
	)

	unpause_specific_operation {
	}: _(
		RawOrigin::Root,
		CurrencyId::DOT,
		Operation::Deposit
	)

	set_insurance_factor {
	}: _(
		RawOrigin::Root,
		CurrencyId::DOT,
		Rate::one()
	)

	set_max_borrow_rate {
	}: _(
		RawOrigin::Root,
		CurrencyId::DOT,
		Rate::one()
	)

	set_collateral_factor {}: _(
		RawOrigin::Root,
		CurrencyId::DOT,
		Rate::one()
	)

	set_borrow_cap {}: _(
		RawOrigin::Root,
		CurrencyId::DOT,
		Some(0u128)
	)

	switch_mode {}: _(
		RawOrigin::Root
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::new_test_ext;
	use frame_support::assert_ok;

	#[test]
	fn test_pause_specific_operation() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_pause_specific_operation());
		})
	}

	#[test]
	fn test_unpause_specific_operation() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_unpause_specific_operation());
		})
	}

	#[test]
	fn test_set_insurance_factor() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_insurance_factor());
		})
	}

	#[test]
	fn test_set_max_borrow_rate() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_max_borrow_rate());
		})
	}

	#[test]
	fn test_set_collateral_factor() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_collateral_factor());
		})
	}

	#[test]
	fn test_set_borrow_cap() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_borrow_cap());
		})
	}

	#[test]
	fn test_switch_mode() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_switch_mode());
		})
	}
}
