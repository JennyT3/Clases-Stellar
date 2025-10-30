#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn create_token_contract<'a>(env: &Env) -> TokenBDBClient<'a> {
    TokenBDBClient::new(env, &env.register_contract(None, TokenBDB {}))
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);

    let name = String::from_str(&env, "Buen Dia Token");
    let symbol = String::from_str(&env, "BDB");
    let decimals = 7u32;

    contract.initialize(&admin, &name, &symbol, &decimals);

    assert_eq!(contract.name(), name);
    assert_eq!(contract.symbol(), symbol);
    assert_eq!(contract.decimals(), decimals);
    assert_eq!(contract.total_supply(), 0);
}

#[test]
fn test_mint_and_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let name = String::from_str(&env, "Buen Dia Token");
    let symbol = String::from_str(&env, "BDB");

    contract.initialize(&admin, &name, &symbol, &7);
    contract.mint(&user, &1000);

    assert_eq!(contract.balance(&user), 1000);
    assert_eq!(contract.total_supply(), 1000);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let name = String::from_str(&env, "Buen Dia Token");
    let symbol = String::from_str(&env, "BDB");

    contract.initialize(&admin, &name, &symbol, &7);
    contract.mint(&user1, &1000);
    contract.transfer(&user1, &user2, &300);

    assert_eq!(contract.balance(&user1), 700);
    assert_eq!(contract.balance(&user2), 300);
    assert_eq!(contract.total_supply(), 1000);
}

#[test]
#[should_panic]
fn test_cannot_reinitialize() {
    let env = Env::default();
    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);

    let name = String::from_str(&env, "Buen Dia Token");
    let symbol = String::from_str(&env, "BDB");

    contract.initialize(&admin, &name, &symbol, &7);
    contract.initialize(&admin, &name, &symbol, &7);
}

#[test]
#[should_panic]
fn test_initialize_empty_name_fails() {
    let env = Env::default();
    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);

    let empty_name = String::from_str(&env, "");
    let symbol = String::from_str(&env, "BDB");

    contract.initialize(&admin, &empty_name, &symbol, &7);
}

#[test]
#[should_panic]
fn test_initialize_empty_symbol_fails() {
    let env = Env::default();
    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);

    let name = String::from_str(&env, "Buen Dia Token");
    let empty_symbol = String::from_str(&env, "");

    contract.initialize(&admin, &name, &empty_symbol, &7);
}

#[test]
#[should_panic]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_token_contract(&env);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let name = String::from_str(&env, "Buen Dia Token");
    let symbol = String::from_str(&env, "BDB");

    contract.initialize(&admin, &name, &symbol, &7);
    contract.mint(&user1, &100);
    contract.transfer(&user1, &user2, &200);
}
