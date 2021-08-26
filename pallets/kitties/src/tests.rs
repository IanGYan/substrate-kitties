use crate::mock::*;
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesCount::<Test>::get(), Some(1));
	});
}

#[test]
fn create_failed_when_kittiescount_overflow() {
	new_test_ext().execute_with(|| {
		KittiesCount::<Test>::put(u32::max_value());
		let account_id: u64 = 1;
		assert_noop!(KittiesModule::create(Origin::signed(account_id)), Error::<Test>::KittiesCountOverflow);
	});
}