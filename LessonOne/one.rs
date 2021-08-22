
#[test]
fn create_claim_works(){
	new_test_ext().execute_with(||{
		let claim= vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::sigend(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),Some((1,frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn create_claim_works_exist(){
	new_test_ext().execute_with(||{
		let claim= vec![0,1];
		let _=PoeModule::create_claim(Origin::sigend(1),claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::sigend(1),claim.clone()),Error::<Test>::ProofAlreadyExist);
	})
}




#[test]
fn revoke_claim_works(){
	new_test_ext().execute_with(||{
		let claim= vec![0,1];
		let _=PoeModule::create_claim(Origin::sigend(1),claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::sigend(1),claim.clone()));

		assert_eq!(Proofs::<Test>::get(&claim),None);
	})
}

#[test]
fn revoke_claim_works_exist(){
	new_test_ext().execute_with(||{
		let claim= vec![0,1];
		assert_noop!(PoeModule::revoke_claim(Origin::sigend(1),claim.clone()),Error::<Test>::ClaimNotExist);
	})
}


#[test]
fn transfer_claim_works(){
	new_test_ext().execute_with(||{
		let claim= vec![0,1];
		assert_ok!(PoeModule::transfer_claim(Origin::sigend(1),claim.clone(),2));
		assert_eq!(Proofs::<Test>::get(&claim),Some((2,frame_system::Pallet::<Test>::block_number())));
	})
}

