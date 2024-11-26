#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, log, Address, String, Env, Symbol, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotOwner = 1,
    NoOwner = 2,
}

#[contract]
pub struct NFTMintContract;

#[contractimpl]
impl NFTMintContract {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: Symbol) {
        log!(
            &env,
            "admin: {}, name: {}, symbol: {}",
            admin,
            name,
            symbol 
        );
    }

    pub fn mint(env: Env, owner: Address, imghash: String) -> String {
        if env.storage().persistent().has(&imghash) {
            panic!("image hash is already minted");
        }
        
        // Store the image hash -> owner mapping
        env.storage().persistent().set(&imghash, &owner);
        
        // Update the owner -> image hashes mapping
        let mut owner_hashes: Vec<String> = env.storage()
            .persistent()
            .get(&owner)
            .unwrap_or(Vec::new(&env));
        
        owner_hashes.push_back(imghash.clone());
        env.storage().persistent().set(&owner, &owner_hashes);
        
        imghash
    }

    pub fn owner_of(env: Env, imghash: String) -> Option<Address> {
        env.storage().persistent().get(&imghash)
    }

    pub fn transfer(env: Env, imghash: String, from: Address, to: Address) -> Result<String, Error> {
        let current_owner: Option<Address> = env.storage().persistent().get(&imghash);
        log!(&env, "check current_owner: {}", current_owner);
        
        match current_owner {
            Some(owner_address) => {
                if owner_address != from {
                    return Err(Error::NotOwner);
                }
            }
            None => return Err(Error::NoOwner),
        }

        // Remove the image hash from the previous owner's list
        let mut from_hashes: Vec<String> = env.storage()
            .persistent()
            .get(&from)
            .unwrap_or(Vec::new(&env));
        
        // Fixed: Comparing String with String (not &String)
        if let Some(index) = from_hashes.iter().position(|x| x.clone() == imghash) {
            // Fixed: Converting usize to u32 for remove_unchecked
            from_hashes.remove_unchecked(index as u32);
        }
        env.storage().persistent().set(&from, &from_hashes);

        // Add the image hash to the new owner's list
        let mut to_hashes: Vec<String> = env.storage()
            .persistent()
            .get(&to)
            .unwrap_or(Vec::new(&env));
        
        to_hashes.push_back(imghash.clone());
        env.storage().persistent().set(&to, &to_hashes);

        // Update the image hash -> owner mapping
        env.storage().persistent().set(&imghash, &to);
        
        let current_owner: Option<Address> = env.storage().persistent().get(&imghash);
        log!(&env, "check current_owner: {}", current_owner);
        
        Ok(imghash)
    }

    pub fn get_image_hashes(env: Env, owner: Address) -> Vec<String> {
        env.storage()
            .persistent()
            .get(&owner)
            .unwrap_or(Vec::new(&env))
    }
}

//alice-keys: GBAYQ5ILEARAQIO26RRKK7XLN2JP37SM6AKJHGVTQXKZFU56TR2Y4GRP

//contract-deployed address: CAJTWAV32QMUBUURZFVTMPIA4SHG3LJQVS2ASFBFIEWU7XQORYW7LHKF