#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, String, Vec, contracttype};

#[contracttype]
#[derive(Clone)]
pub struct Shelter {
    pub name: String,
    pub location: String,
    pub capacity: u32,
    pub is_active: bool,
}

const COUNT_SHELTER: Symbol = symbol_short!("COUNT");
#[contracttype]
pub enum ShelterBook {
    Shelter(u32),
}

#[contract]
pub struct ShelterLocatorContract;

#[contractimpl]
impl ShelterLocatorContract {
    pub fn register_shelter(env: Env, name: String, location: String, capacity: u32) -> u32 {
        let mut count = env.storage().instance().get(&COUNT_SHELTER).unwrap_or(0u32);
        count += 1;

        let shelter = Shelter {
            name,
            location,
            capacity,
            is_active: true,
        };

        env.storage().instance().set(&ShelterBook::Shelter(count), &shelter);
        env.storage().instance().set(&COUNT_SHELTER, &count);
        count
    }

    pub fn get_shelter(env: Env, id: u32) -> Shelter {
        env.storage().instance()
            .get(&ShelterBook::Shelter(id))
            .unwrap_or(Shelter {
                name: String::from_str(&env, "Not Found"),
                location: String::from_str(&env, "Not Found"),
                capacity: 0,
                is_active: false,
            })
    }

    pub fn toggle_shelter_status(env: Env, id: u32, status: bool) {
        let mut shelter = Self::get_shelter(env.clone(), id);
        shelter.is_active = status;
        env.storage().instance().set(&ShelterBook::Shelter(id), &shelter);
    }
}
