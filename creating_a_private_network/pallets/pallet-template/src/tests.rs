use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(PalletTemplate::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(PalletTemplate::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			PalletTemplate::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn correct_error_for_unsigned_origin_while_doing_something_with_correct_details() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            PalletTemplate::do_something(Origin::none(), 30),
            DispatchError::BadOrigin,
        );
    });
}
