use crate::mock::{Event as TestEvent, new_test_ext, KittiesModule, Origin, System, Test};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesCount::<Test>::get(), Some(1));
		assert_eq!(Owner::<Test>::get(0), Some(1));
		// Test the Event emitted already.
		// Event::<Test>::KittyCreated(Owner, KittyIndex)
		assert_has_event!(Event::<Test>::KittyCreated(1,0));
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

#[test]
fn create_failed_when_not_enough_balance_for_staking() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 3;
		assert_noop!(KittiesModule::create(Origin::signed(account_id)), Error::<Test>::NotEnoughBalanceForStaking);

	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		// Prepare a kitty index=0, by AccountID =1.
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		// Transfer AccountID 1 to AccountID 2, KittyIndex = 0
		assert_ok!(KittiesModule::transfer(Origin::signed(account_id), 2, 0));
		assert_eq!(Owner::<Test>::get(0), Some(2));
		// Test the Event emitted already.
		// KittyTransferred(Owner, New Owner, KittyIndex)
		assert_has_event!(Event::<Test>::KittyTransferred(1, 2, 0));
	});
}

#[test]
fn transfer_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		// Prepare a kitty index=0, by AccountID =1.
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		// Transfer AccountID 2 (not owner) to AccountID 3, KittyIndex = 0
		assert_noop!(KittiesModule::transfer(Origin::signed(2u64), 3, 0), Error::<Test>::NotOwner);
	});
}

#[test]
fn transfer_failed_when_new_owner_not_enough_balance() {
	new_test_ext().execute_with(|| {
		// Prepare a kitty index=0, by AccountID =1.
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		// Transfer AccountID 1 (the owner) to AccountID 3, KittyIndex = 0
		assert_noop!(KittiesModule::transfer(Origin::signed(account_id), 3, 0), Error::<Test>::NotEnoughBalanceForStaking);
	});
}

#[test]
fn breed_works() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// Prepare kitty index=1, by AccountID =2.
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		// Breed a kitty index=2 from 0&1, by AccountID =1.
		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
		assert_eq!(KittiesCount::<Test>::get(), Some(3));
		// Test the Event emitted already.
		// Event::<Test>::KittyCreated(Owner, KittyIndex)
		assert_has_event!(Event::<Test>::KittyCreated(1, 2));
	});
}

#[test]
fn breed_failed_when_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// Prepare kitty index=1, by AccountID =2.
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		// No KittyID =3.
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 3), Error::<Test>::InvalidKittyIndex);
	});
}

#[test]
fn breed_failed_when_same_parent() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// Prepare kitty index=1, by AccountID =2.
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		// Parent Index shouldn't be same.
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameParentIndex);
	});
}

#[test]
fn breed_failed_when_not_enough_balance_for_staking() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// Prepare kitty index=1, by AccountID =2.
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		// Account 3 has not enough balance for staing
		assert_noop!(KittiesModule::breed(Origin::signed(3), 0, 1), Error::<Test>::NotEnoughBalanceForStaking);
	});
}

#[test]
fn sell_works() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// List Kitty index=0 for sale with a price=1_500.
		let price: u128 = 1_500;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		assert_eq!(ListForSale::<Test>::get(0), Some(price));
		// Test the Event emitted already.
		// KittyListed(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>)
		assert_has_event!(Event::<Test>::KittyListed(1, 0, Some(price)));
	});
}

#[test]
fn sell_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// List Kitty index=0 for sale with a price=1_500, but not by the owner.
		let price: u128 = 1_500;
		assert_noop!(KittiesModule::sell(Origin::signed(3), 0, Some(price)), Error::<Test>::NotOwner);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// List Kitty index=0 for sale with a price=1_500.
		let price: u128 = 1_500;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		// AccountID=2 buy KittyIndex=0 (from AccountID=1)
		assert_ok!(KittiesModule::buy(Origin::signed(2), 0));
		assert_eq!(Owner::<Test>::get(0), Some(2));
		// Test the Event emitted.
		// KittyTransferred(Seller, Buyer, KittyIndex)
		assert_has_event!(Event::<Test>::KittyTransferred(1, 2, 0));
	});
}

#[test]
fn buy_failed_when_buyer_is_owner() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// List Kitty index=0 for sale with a price=1_500.
		let price: u128 = 1_500;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		// AccountID=1 (is owner) buy KittyIndex=0 (from AccountID=1)
		assert_noop!(KittiesModule::buy(Origin::signed(1), 0), Error::<Test>::BuyerIsOwner);
	});
}

#[test]
fn buy_failed_when_not_for_sale() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// List Kitty index=0 for sale with a price=None, which means not for sale.
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, None));
		// AccountID=2 buy KittyIndex=0 (from AccountID=1), but the kitty is not for sale.
		assert_noop!(KittiesModule::buy(Origin::signed(2), 0), Error::<Test>::NotForSale);
	});
}

#[test]
fn buy_failed_when_not_enough_balance() {
	new_test_ext().execute_with(|| {
		// Prepare kitty index=0, by AccountID =1.
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		// List Kitty index=0 for sale with a price=1_500.
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(1_500)));
		// AccountID=3 (who is poor) buy KittyIndex=0 (from AccountID=1).
		assert_noop!(KittiesModule::buy(Origin::signed(3), 0), Error::<Test>::NotEnoughBalanceForBuying);
	});
}
