#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_create_donation() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ShareCycleContract);
    let client = ShareCycleContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);

    let item_id = client.create_donation(
        &donor,
        &String::from_str(&env, "Nursing Book"),
        &String::from_str(&env, "2nd year nursing reference"),
    );

    assert_eq!(item_id, 1);
}

#[test]
fn test_request_item() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ShareCycleContract);
    let client = ShareCycleContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    let item_id = client.create_donation(
        &donor,
        &String::from_str(&env, "Calculator"),
        &String::from_str(&env, "Engineering calculator"),
    );

    client.request_item(&beneficiary, &item_id);

    let donation = client.get_donation(&item_id);

    assert_eq!(donation.claimed, true);
}

#[test]
fn test_confirm_received() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ShareCycleContract);
    let client = ShareCycleContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);

    let item_id = client.create_donation(
        &donor,
        &String::from_str(&env, "Laptop"),
        &String::from_str(&env, "Old student laptop"),
    );

    client.confirm_received(&donor, &item_id);

    let donation = client.get_donation(&item_id);

    assert_eq!(donation.completed, true);
}

#[test]
#[should_panic]
fn test_duplicate_claim() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ShareCycleContract);
    let client = ShareCycleContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);
    let beneficiary1 = Address::generate(&env);
    let beneficiary2 = Address::generate(&env);

    let item_id = client.create_donation(
        &donor,
        &String::from_str(&env, "Uniform"),
        &String::from_str(&env, "Nursing duty uniform"),
    );

    client.request_item(&beneficiary1, &item_id);

    client.request_item(&beneficiary2, &item_id);
}

#[test]
fn test_storage_verification() {

    let env = Env::default();

    let contract_id = env.register_contract(None, ShareCycleContract);
    let client = ShareCycleContractClient::new(&env, &contract_id);

    let donor = Address::generate(&env);

    let item_id = client.create_donation(
        &donor,
        &String::from_str(&env, "Books"),
        &String::from_str(&env, "Bundle of reviewers"),
    );

    let donation = client.get_donation(&item_id);

    assert_eq!(donation.item_id, item_id);
}