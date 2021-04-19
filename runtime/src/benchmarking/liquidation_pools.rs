use super::utils::set_balance;
use crate::{DexModuleId, LiquidationPools, LiquidationPoolsModuleId, Rate, Runtime, DOLLARS, DOT, ETH};
use frame_system::RawOrigin;
use orml_benchmarking::runtime_benchmarks;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::FixedPointNumber;
use sp_std::prelude::*;

runtime_benchmarks! {
	{ Runtime, liquidation_pools }

	_ {}

	set_balancing_period {}: _(RawOrigin::Root, 100)
	verify { assert_eq!(LiquidationPools::balancing_period(), 100) }

	set_deviation_threshold {}: _(RawOrigin::Root, DOT, 10u128.pow(18))
	verify { assert_eq!(LiquidationPools::liquidation_pools_data(DOT).deviation_threshold, Rate::one()) }

	set_balance_ratio {}: _(RawOrigin::Root, DOT,  10u128.pow(18))
	verify { assert_eq!(LiquidationPools::liquidation_pools_data(DOT).balance_ratio, Rate::one()) }

	set_max_ideal_balance {}: _(RawOrigin::Root, DOT,  Some(10u128.pow(18)))
	verify { assert_eq!(LiquidationPools::liquidation_pools_data(DOT).max_ideal_balance, Some(10u128.pow(18))) }

	balance_liquidation_pools {
		set_balance(
			ETH,
			&DexModuleId::get().into_account(),
			20_000 * DOLLARS,
		)?;
		set_balance(
			DOT,
			&LiquidationPoolsModuleId::get().into_account(),
			20_000 * DOLLARS,
		)?;
	}: _(RawOrigin::None, DOT, ETH, 10_000 * DOLLARS, 10_000 * DOLLARS)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::test_externalities;
	use frame_support::assert_ok;

	#[test]
	fn test_set_balancing_period() {
		test_externalities().execute_with(|| {
			assert_ok!(test_benchmark_set_balancing_period());
		})
	}

	#[test]
	fn test_set_deviation_threshold() {
		test_externalities().execute_with(|| {
			assert_ok!(test_benchmark_set_deviation_threshold());
		})
	}

	#[test]
	fn test_set_balance_ratio() {
		test_externalities().execute_with(|| {
			assert_ok!(test_benchmark_set_balance_ratio());
		})
	}

	#[test]
	fn test_set_max_ideal_balance() {
		test_externalities().execute_with(|| {
			assert_ok!(test_benchmark_set_max_ideal_balance());
		})
	}

	#[test]
	fn test_balance_liquidation_pools() {
		test_externalities().execute_with(|| {
			assert_ok!(test_benchmark_balance_liquidation_pools());
		})
	}
}
