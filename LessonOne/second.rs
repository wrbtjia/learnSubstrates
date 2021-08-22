
#[test]
fn create_claim_works_range(){
	new_test_ext().execute_with(||{
		let claim= vec![0,1,1,1,1,1,1,1,1,];
		assert_noop!(PoeModule::create_claim(Origin::sigend(1),claim.clone()),Error::<Test>::ClaimLengthOutRange);
	})
}



