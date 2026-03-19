#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

// Storage key
const REPUTATION: Symbol = symbol_short!("REP");

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {

    // Initialize reputation map
    pub fn init(env: Env) {
        let map: Map<Address, i32> = Map::new(&env);
        env.storage().instance().set(&REPUTATION, &map);
    }

    // Add reputation to a user
    pub fn add_rep(env: Env, user: Address, amount: i32) {
        let mut map: Map<Address, i32> =
            env.storage().instance().get(&REPUTATION).unwrap();

        let current = map.get(user.clone()).unwrap_or(0);
        let updated = current + amount;

        map.set(user, updated);
        env.storage().instance().set(&REPUTATION, &map);
    }

    // Deduct reputation
    pub fn subtract_rep(env: Env, user: Address, amount: i32) {
        let mut map: Map<Address, i32> =
            env.storage().instance().get(&REPUTATION).unwrap();

        let current = map.get(user.clone()).unwrap_or(0);
        let updated = current - amount;

        map.set(user, updated);
        env.storage().instance().set(&REPUTATION, &map);
    }

    // Get reputation of a user
    pub fn get_rep(env: Env, user: Address) -> i32 {
        let map: Map<Address, i32> =
            env.storage().instance().get(&REPUTATION).unwrap();

        map.get(user).unwrap_or(0)
    }
}