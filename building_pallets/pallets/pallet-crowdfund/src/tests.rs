use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};

#[test]
fn correct_error_for_unsigned_origin_while_creating_task_with_correct_() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            PalletCrowdfund::create(Origin::none(), 123456789, 30000, 10),
            DispatchError::BadOrigin,
        );
    });
}

// #[test]
// fn it_works_when_creating_with_correct_details() {
//     new_test_ext().execute_with(|| {
//         // Ensure the expected error is thrown when no value is present.
//         assert_ok!(PalletCrowdfund::create(
//             Origin::signed(1),
//             123456789,
//             30000,
//             10
//         ));
//     });
// }
