#![no_std]

mod storage;
mod errors;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String};
use storage::{DataKey, TokenMetadata};
use errors::TokenError;

const MAX_NAME_LENGTH: u32 = 32;
const MAX_SYMBOL_LENGTH: u32 = 12;

#[contract]
pub struct TokenBDB;

#[contractimpl]
impl TokenBDB {
    /// Inicializa el token con metadatos y un administrador
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) -> Result<(), TokenError> {
        // Verificar que no esté ya inicializado
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(TokenError::AlreadyInitialized);
        }

        // Validar metadatos
        if name.len() == 0 || name.len() > MAX_NAME_LENGTH {
            return Err(TokenError::InvalidMetadata);
        }
        if symbol.len() == 0 || symbol.len() > MAX_SYMBOL_LENGTH {
            return Err(TokenError::InvalidMetadata);
        }
        if decimals > 18 {
            return Err(TokenError::InvalidMetadata);
        }

        // Guardar admin
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Guardar metadatos
        let metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            decimals,
        };
        env.storage().instance().set(&DataKey::Metadata, &metadata);

        // Inicializar supply total en 0
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);

        Ok(())
    }

    /// Mintea nuevos tokens a una dirección
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        // Verificar que esté inicializado
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(TokenError::NotInitialized)?;

        // Solo el admin puede mintear
        admin.require_auth();

        // Validar amount
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        // Obtener balance actual
        let balance_key = DataKey::Balance(to.clone());
        let current_balance: i128 = env
            .storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);

        // Actualizar balance
        let new_balance = current_balance + amount;
        env.storage().persistent().set(&balance_key, &new_balance);

        // Actualizar supply total
        let current_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(current_supply + amount));

        Ok(())
    }

    /// Transfiere tokens de una dirección a otra
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), TokenError> {
        // Verificar que esté inicializado
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(TokenError::NotInitialized);
        }

        // El remitente debe autorizar
        from.require_auth();

        // Validar amount
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        // Obtener balance del remitente
        let from_key = DataKey::Balance(from.clone());
        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&from_key)
            .ok_or(TokenError::InsufficientBalance)?;

        // Verificar balance suficiente
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        // Actualizar balance del remitente
        env.storage()
            .persistent()
            .set(&from_key, &(from_balance - amount));

        // Obtener y actualizar balance del receptor
        let to_key = DataKey::Balance(to.clone());
        let to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
        env.storage()
            .persistent()
            .set(&to_key, &(to_balance + amount));

        Ok(())
    }

    /// Consulta el balance de una dirección
    pub fn balance(env: Env, account: Address) -> i128 {
        let key = DataKey::Balance(account);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Consulta el supply total
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Consulta el nombre del token
    pub fn name(env: Env) -> Result<String, TokenError> {
        let metadata: TokenMetadata = env
            .storage()
            .instance()
            .get(&DataKey::Metadata)
            .ok_or(TokenError::NotInitialized)?;
        Ok(metadata.name)
    }

    /// Consulta el símbolo del token
    pub fn symbol(env: Env) -> Result<String, TokenError> {
        let metadata: TokenMetadata = env
            .storage()
            .instance()
            .get(&DataKey::Metadata)
            .ok_or(TokenError::NotInitialized)?;
        Ok(metadata.symbol)
    }

    /// Consulta los decimales del token
    pub fn decimals(env: Env) -> Result<u32, TokenError> {
        let metadata: TokenMetadata = env
            .storage()
            .instance()
            .get(&DataKey::Metadata)
            .ok_or(TokenError::NotInitialized)?;
        Ok(metadata.decimals)
    }
}
