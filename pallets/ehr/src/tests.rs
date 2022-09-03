use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::inherent::Vec;

use sp_runtime::offchain::testing;
use sp_runtime::{
	traits::{BadOrigin},
};
#[test]
fn add_cloud_provider_works() {
	let (mut t, _, _) = ExternalityBuilder::build();
	t.execute_with(|| {
		assert_noop!(Ehr::add_provider(Origin::signed(test_pub(5)), test_pub(15)), BadOrigin);
		assert_ok!(Ehr::add_provider(Origin::root(), test_pub(15)));
 		assert_eq!(Ehr::providers(test_pub(15)), Vec::<crate::mock::AccountId>::new());
});
}
#[test]
fn remove_cloud_provider_works() {
	let (mut t, _, _) = ExternalityBuilder::build();
	t.execute_with(|| {
		assert_ok!(Ehr::add_provider(Origin::root(), test_pub(15)));
		assert_noop!(
			Ehr::remove_provider(Origin::signed(test_pub(2)), test_pub(15)),
			BadOrigin
		);
		assert_ok!(Ehr::remove_provider(Origin::root(), test_pub(15)));
 		assert_eq!(Ehr::providers(test_pub(15)), Vec::<AccountId>::new()); 
});
}
#[test]
fn add_patient_works() {
	let (mut t, _, _) = ExternalityBuilder::build();
	t.execute_with(|| {
		assert_noop!(Ehr::add_patient(Origin::signed(test_pub(5)), test_pub(15)), BadOrigin);
		assert_ok!(Ehr::add_patient(Origin::root(), test_pub(15)));
 		assert_eq!(Ehr::patients(test_pub(15)), Vec::new());
});
}
#[test]
fn remove_patient_works() {
	let (mut t, _, offchain_state) = ExternalityBuilder::build();
	t.execute_with(|| {
		assert_ok!(Ehr::add_patient(Origin::root(), test_pub(15)));
		assert_ok!(Ehr::remove_patient(Origin::root(), test_pub(15)));
 		assert_eq!(Ehr::patients(test_pub(15)), vec![]);
});
}

#[test]
fn get_access_token_works() {
	let (mut t, _, offchain_state) = ExternalityBuilder::build();
	{
		let mut state = offchain_state.write();
			state.expect_request(testing::PendingRequest {
				method: "POST".into(),
				uri: "http://localhost:8001".into(),
				response: Some(b"thisisyourtoken".to_vec()),
				sent: true,
				..Default::default()
			});
	}
	t.execute_with(|| {
		let token_url = b"http://localhost:8001".to_vec();
		let client_id = b"http://localhost:8003".to_vec();
		let kid = b"09f1fa89-c4ea-4d81-aa21-2ad3c781435c".to_vec();
		let pem = include_bytes!("ec_p384_private.pem").to_vec();
		println!("{:?}",pem);
		let result = Ehr::create_access_token(token_url, client_id, kid, pem).unwrap();
 		assert_eq!(result, b"thisisyourtoken".to_vec());
});
}


