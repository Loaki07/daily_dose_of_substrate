use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(PalletCrowdfund::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(PalletCrowdfund::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			PalletCrowdfund::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn correct_error_for_unsigned_origin_while_creating_task_with_correct_() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            PalletCrowdfund::create_task(Origin::none(), 30, 300, b"Create a website".to_vec()),
            DispatchError::BadOrigin,
        );
    });
}
