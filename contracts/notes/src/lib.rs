#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Vec, symbol_short
};

#[derive(Clone)]
#[contracttype]
pub struct Booking {
    pub owner: Address,
    pub booker: Option<Address>,
    pub price: i128,
    pub status: u32, // 0 = available, 1 = booked, 2 = completed
    pub created_at: u64,
    pub booked_at: Option<u64>,
}

#[contract]
pub struct SelfBookContract;

#[contractimpl]
impl SelfBookContract {

    const MAX_SLOTS: u32 = 20;

    // Create slot
    pub fn create_slot(env: Env, owner: Address, price: i128) {
        owner.require_auth();

        if price <= 0 {
            panic!("Price must be positive");
        }

        let mut slots: Vec<Booking> =
            env.storage()
                .instance()
                .get(&symbol_short!("slots"))
                .unwrap_or(Vec::new(&env));

        if slots.len() >= SelfBookContract::MAX_SLOTS {
            panic!("Slot limit reached");
        }

        let booking = Booking {
            owner: owner.clone(),
            booker: None,
            price,
            status: 0,
            created_at: env.ledger().timestamp(),
            booked_at: None,
        };

        slots.push_back(booking);

        env.storage()
            .instance()
            .set(&symbol_short!("slots"), &slots);
    }

    // Book slot
    pub fn book_slot(env: Env, user: Address, index: u32) {
        user.require_auth();

        let mut slots: Vec<Booking> =
            env.storage()
                .instance()
                .get(&symbol_short!("slots"))
                .unwrap_or(Vec::new(&env));

        if index >= slots.len() {
            panic!("Invalid slot");
        }

        let mut slot = slots.get(index).unwrap();

        if slot.status != 0 {
            panic!("Slot not available");
        }

        if slot.owner == user {
            panic!("Owner cannot book own slot");
        }

        slot.status = 1;
        slot.booker = Some(user.clone());
        slot.booked_at = Some(env.ledger().timestamp());

        slots.set(index, slot);

        env.storage()
            .instance()
            .set(&symbol_short!("slots"), &slots);
    }

    // Complete booking (only owner)
    pub fn complete_booking(env: Env, owner: Address, index: u32) {
        owner.require_auth();

        let mut slots: Vec<Booking> =
            env.storage()
                .instance()
                .get(&symbol_short!("slots"))
                .unwrap_or(Vec::new(&env));

        if index >= slots.len() {
            panic!("Invalid slot");
        }

        let mut slot = slots.get(index).unwrap();

        if slot.owner != owner {
            panic!("Not owner");
        }

        if slot.status != 1 {
            panic!("Slot not booked");
        }

        slot.status = 2;

        slots.set(index, slot);

        env.storage()
            .instance()
            .set(&symbol_short!("slots"), &slots);
    }

    // Get all slots
    pub fn get_slots(env: Env) -> Vec<Booking> {
        env.storage()
            .instance()
            .get(&symbol_short!("slots"))
            .unwrap_or(Vec::new(&env))
    }

    // Get slots by owner
    pub fn get_slots_by_owner(env: Env, owner: Address) -> Vec<Booking> {
        let slots: Vec<Booking> =
            env.storage()
                .instance()
                .get(&symbol_short!("slots"))
                .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for slot in slots.iter() {
            if slot.owner == owner {
                result.push_back(slot);
            }
        }

        result
    }
}