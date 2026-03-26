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
    pub booked: bool,
}

#[contract]
pub struct SelfBookContract;

#[contractimpl]
impl SelfBookContract {

    const MAX_SLOTS: u32 = 20;

    // Create booking slot
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
            booked: false,
        };

        slots.push_back(booking);

        env.storage()
            .instance()
            .set(&symbol_short!("slots"), &slots);

        env.events().publish(
            (symbol_short!("create"), owner),
            price
        );
    }

    // Book a slot
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

        if slot.booked {
            panic!("Already booked");
        }

        if slot.owner == user {
            panic!("Owner cannot book own slot");
        }

        slot.booked = true;
        slot.booker = Some(user.clone());

        slots.set(index, slot.clone());

        env.storage()
            .instance()
            .set(&symbol_short!("slots"), &slots);

        env.events().publish(
            (symbol_short!("book"), user),
            index
        );
    }

    // Cancel booking (only booker)
    pub fn cancel_booking(env: Env, user: Address, index: u32) {
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

        if !slot.booked {
            panic!("Slot not booked");
        }

        if slot.booker != Some(user.clone()) {
            panic!("Not authorized");
        }

        slot.booked = false;
        slot.booker = None;

        slots.set(index, slot.clone());

        env.storage()
            .instance()
            .set(&symbol_short!("slots"), &slots);

        env.events().publish(
            (symbol_short!("cancel"), user),
            index
        );
    }

    // Get all slots
    pub fn get_slots(env: Env) -> Vec<Booking> {
        env.storage()
            .instance()
            .get(&symbol_short!("slots"))
            .unwrap_or(Vec::new(&env))
    }
}