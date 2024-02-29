use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_works_for_flip() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(FlipModule::flip(RuntimeOrigin::signed(1)));
		// Read pallet storage and assert an expected result.
		assert_eq!(FlipModule::flipper(), true);
		System::assert_last_event(
			crate::Event::Flipped { who: Some(1), value: false }.into(),
		);
	});
}
