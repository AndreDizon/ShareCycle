#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, String, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct DonationItem {
    pub item_id: u64,
    pub donor: Address,
    pub item_name: String,
    pub description: String,
    pub beneficiary: Address,
    pub claimed: bool,
    pub completed: bool,
}

#[contracttype]
pub enum DataKey {
    Donation(u64),
    Counter,
}

#[contract]
pub struct ShareCycleContract;

#[contractimpl]
impl ShareCycleContract {

    // Create donation listing
    pub fn create_donation(
        env: Env,
        donor: Address,
        item_name: String,
        description: String,
    ) -> u64 {

        donor.require_auth();

        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let donation = DonationItem {
            item_id: counter,
            donor: donor.clone(),
            item_name,
            description,
            beneficiary: donor.clone(),
            claimed: false,
            completed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Donation(counter), &donation);

        env.storage()
            .instance()
            .set(&DataKey::Counter, &counter);

        counter
    }

    // Beneficiary requests item
    pub fn request_item(
        env: Env,
        beneficiary: Address,
        item_id: u64,
    ) {

        beneficiary.require_auth();

        let key = DataKey::Donation(item_id);

        let mut donation: DonationItem = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if donation.claimed {
            panic!("Item already claimed");
        }

        donation.beneficiary = beneficiary;
        donation.claimed = true;

        env.storage()
            .persistent()
            .set(&key, &donation);
    }

    // Donor confirms transfer completed
    pub fn confirm_received(
        env: Env,
        donor: Address,
        item_id: u64,
    ) {

        donor.require_auth();

        let key = DataKey::Donation(item_id);

        let mut donation: DonationItem = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if donation.donor != donor {
            panic!("Unauthorized donor");
        }

        donation.completed = true;

        env.storage()
            .persistent()
            .set(&key, &donation);
    }

    // View donation details
    pub fn get_donation(
        env: Env,
        item_id: u64,
    ) -> DonationItem {

        env.storage()
            .persistent()
            .get(&DataKey::Donation(item_id))
            .unwrap()
    }
}