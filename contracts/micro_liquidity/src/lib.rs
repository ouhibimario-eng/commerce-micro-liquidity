
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PoolError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidTarget = 3,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    FundingTarget,
}

#[contract]
pub struct MicroLiquidityPool;

#[contractimpl]
impl MicroLiquidityPool {
    /// Initialize the pool configuration once.
    /// This function does not transfer any funds.
    pub fn initialize(
        env: Env,
        admin: Address,
        funding_target: i128,
    ) -> Result<(), PoolError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(PoolError::AlreadyInitialized);
        }

        if funding_target <= 0 {
            return Err(PoolError::InvalidTarget);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::FundingTarget, &funding_target);

        Ok(())
    }

    /// Return the configured administrator.
    pub fn get_admin(env: Env) -> Result<Address, PoolError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(PoolError::NotInitialized)
    }

    /// Return the funding target.
    pub fn get_funding_target(env: Env) -> Result<i128, PoolError> {
        env.storage()
            .instance()
            .get(&DataKey::FundingTarget)
            .ok_or(PoolError::NotInitialized)
    }
}
