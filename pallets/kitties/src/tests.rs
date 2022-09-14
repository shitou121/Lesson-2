use std::ops::Add;

use super::*;
use frame_support::{assert_noop, assert_ok, traits::ConstU64, BoundedVec};
use mock::{new_test_ext, Event as TestEvent, KittiesModule, Origin, System, Test};

#[test]
fn creating_kitty() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 0;
		let kitty_id: u32 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittyOwner::<Test>::get(kitty_id), Some(account_id));
		assert_ne!(Kitties::<Test>::get(kitty_id), None);
		assert_eq!(NextKittyId::<Test>::get(), kitty_id.add(&1));
		assert_eq!(KittyPrices::<Test>::get(kitty_id),Some(0_u128));

	});
}

#[test]
fn breed_kitty(){
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let kitty_id: u32 = NextKittyId::<Test>::get();
		let _ = KittiesModule::create(Origin::signed(account_id_1));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1),kitty_id,kitty_id),
			Error::<Test>::SameKittyId
		);
		assert_noop!(
			KittiesModule::breed(Origin::signed(1),3,4),
			Error::<Test>::InvalidKittyId
		);
	})
}

#[test]
fn transfer_kitty(){
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let kitty_id: u32 = NextKittyId::<Test>::get();
		let _ = KittiesModule::create(Origin::signed(account_id_1));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(2),kitty_id,account_id_1),
			Error::<Test>::NotOwner
		);
	})
}
//sell_set_price(origin: OriginFor<T>, kitty_id: T::KittyIndex, new_price:u128)
#[test]
fn set_price_kitty(){
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let kitty_id: u32 = NextKittyId::<Test>::get();
		let _ = KittiesModule::create(Origin::signed(account_id_1));
		assert_noop!(
			KittiesModule::sell_set_price(Origin::signed(2),kitty_id,122_u128),
			Error::<Test>::NotOwner
		);
	})
}
//pub fn buy_kitty(origin:OriginFor<T>,kitty_id:T::KittyIndex,price:u128) 
fn buy_price_kitty(){
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let kitty_id: u32 = NextKittyId::<Test>::get();
		let _ = KittiesModule::create(Origin::signed(account_id_1));
		let _ = KittiesModule::sell_set_price(Origin::signed(account_id_1),kitty_id,228_u128);
		assert_noop!(
			KittiesModule::buy_kitty(Origin::signed(2),kitty_id,229_u128),
			Error::<Test>::KittyNotPrice
		);
	})
}