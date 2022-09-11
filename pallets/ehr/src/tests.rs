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
/* 
#[test]
fn get_access_token_works() {
	let (mut t, _, offchain_state) = ExternalityBuilder::build();
	{
		let mut state = offchain_state.write();
		state.expect_request(testing::PendingRequest {
			method: "POST".into(),
			headers: vec![("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string()), ("accept".to_string(), "application/json".to_string())],
			uri: "http://localhost:8001".into(),
			body: [115, 99, 111, 112, 101, 61, 115, 121, 115, 116, 101, 109, 47, 42, 46, 42, 38, 103, 114, 97, 110, 116, 95, 116, 121, 112, 101, 61, 99, 108, 105, 101, 110, 116, 95, 99, 114, 101, 100, 101, 110, 116, 105, 97, 108, 115, 38, 99, 108, 105, 101, 110, 116, 95, 97, 115, 115, 101, 114, 116, 105, 111, 110, 95, 116, 121, 112, 101, 61, 117, 114, 110, 58, 105, 101, 116, 102, 58, 112, 97, 114, 97, 109, 115, 58, 111, 97, 117, 116, 104, 58, 99, 108, 105, 101, 110, 116, 45, 97, 115, 115, 101, 114, 116, 105, 111, 110, 45, 116, 121, 112, 101, 58, 106, 119, 116, 45, 98, 101, 97, 114, 101, 114, 38, 99, 108, 105, 101, 110, 116, 95, 97, 115, 115, 101, 114, 116, 105, 111, 110, 61, 101, 121, 74, 48, 101, 88, 65, 105, 79, 105, 74, 75, 86, 49, 81, 105, 76, 67, 74, 104, 98, 71, 99, 105, 79, 105, 74, 70, 85, 122, 77, 52, 78, 67, 73, 115, 73, 109, 116, 112, 90, 67, 73, 54, 73, 106, 65, 53, 90, 106, 70, 109, 89, 84, 103, 53, 76, 87, 77, 48, 90, 87, 69, 116, 78, 71, 81, 52, 77, 83, 49, 104, 89, 84, 73, 120, 76, 84, 74, 104, 90, 68, 78, 106, 78, 122, 103, 120, 78, 68, 77, 49, 89, 121, 74, 57, 46, 101, 121, 74, 104, 100, 87, 81, 105, 79, 105, 74, 111, 100, 72, 82, 119, 79, 105, 56, 118, 98, 71, 57, 106, 89, 87, 120, 111, 98, 51, 78, 48, 79, 106, 103, 119, 77, 68, 69, 105, 76, 67, 74, 108, 101, 72, 65, 105, 79, 106, 89, 119, 77, 68, 65, 119, 76, 67, 74, 112, 99, 51, 77, 105, 79, 105, 74, 111, 100, 72, 82, 119, 79, 105, 56, 118, 98, 71, 57, 106, 89, 87, 120, 111, 98, 51, 78, 48, 79, 106, 103, 119, 77, 68, 77, 105, 76, 67, 74, 113, 100, 71, 107, 105, 79, 105, 73, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 77, 68, 65, 119, 73, 105, 119, 105, 99, 51, 86, 105, 73, 106, 111, 105, 97, 72, 82, 48, 99, 68, 111, 118, 76, 50, 120, 118, 89, 50, 70, 115, 97, 71, 57, 122, 100, 68, 111, 52, 77, 68, 65, 122, 73, 110, 48, 46, 95, 81, 82, 52, 89, 120, 86, 82, 77, 113, 68, 48, 98, 101, 84, 101, 104, 122, 48, 90, 85, 45, 78, 69, 86, 102, 119, 99, 45, 52, 79, 120, 108, 71, 112, 98, 52, 82, 104, 99, 69, 97, 110, 106, 45, 90, 112, 87, 115, 120, 80, 81, 122, 75, 80, 121, 74, 98, 50, 121, 104, 48, 88, 85, 48, 115, 86, 56, 48, 55, 50, 99, 86, 117, 72, 73, 50, 45, 65, 121, 65, 66, 84, 115, 104, 121, 75, 105, 87, 106, 79, 115, 104, 97, 84, 108, 48, 90, 105, 75, 48, 101, 106, 89, 99, 90, 103, 114, 75, 112, 45, 103, 95, 74, 106, 51, 80, 72, 121, 65, 65, 122, 90, 90, 83, 53, 122, 114, 38].to_vec(),
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
		let result = Ehr::create_access_token(token_url, client_id, kid, pem).unwrap();
 		assert_eq!(result, b"thisisyourtoken".to_vec());
});
} */


