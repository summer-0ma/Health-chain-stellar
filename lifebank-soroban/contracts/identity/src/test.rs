#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, BytesN, Env, String};

#[test]
fn test_register_organization() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let name = String::from_str(&env, "City Blood Bank");
    let license = String::from_str(&env, "L12345");
    let location_hash = BytesN::from_array(&env, &[0u8; 32]);
    let doc_hashes = vec![&env, BytesN::from_array(&env, &[1u8; 32])];

    let org_id = client.register_organization(
        &owner,
        &OrgType::BloodBank,
        &name,
        &license,
        &location_hash,
        &doc_hashes,
    );

    assert_eq!(org_id, owner);

    // Verify organization storage
    let org = client.get_organization(&org_id).unwrap();
    assert_eq!(org.name, name);
    assert_eq!(org.license_number, license);
    assert_eq!(org.org_type, OrgType::BloodBank);
    assert_eq!(org.verified, false);

    // Verify role assignment
    let role = client.get_role(&org_id).unwrap();
    assert_eq!(role, Role::BloodBank);
}

#[test]
fn test_register_duplicate_license() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let name = String::from_str(&env, "Org");
    let license = String::from_str(&env, "DUP123");
    let location_hash = BytesN::from_array(&env, &[0u8; 32]);
    let doc_hashes = vec![&env];

    client.register_organization(
        &owner1,
        &OrgType::BloodBank,
        &name,
        &license,
        &location_hash,
        &doc_hashes,
    );

    // Attempt to register another org with the same license
    let result = client.try_register_organization(
        &owner2,
        &OrgType::Hospital,
        &name,
        &license,
        &location_hash,
        &doc_hashes,
    );

    assert_eq!(
        result,
        Err(Ok(Error::LicenseAlreadyRegistered))
    );
}

#[test]
fn test_register_invalid_input() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let empty_name = String::from_str(&env, "");
    let license = String::from_str(&env, "L123");
    let location_hash = BytesN::from_array(&env, &[0u8; 32]);
    let doc_hashes = vec![&env];

    let result = client.try_register_organization(
        &owner,
        &OrgType::BloodBank,
        &empty_name,
        &license,
        &location_hash,
        &doc_hashes,
    );

    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}
