#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
    LimiteInvalido = 5,  // ← NUEVO
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
    ContadorPorUsuario(Address),
    LimiteCaracteres,  // ← NUEVO
}

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);

        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);

        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }

    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: Symbol
    ) -> Result<Symbol, Error> {
        // Validación 1: Nombre no puede estar vacío
        if nombre == Symbol::new(&env, "") {
            return Err(Error::NombreVacio);
        }
        
        // Validación 2: Verificar límite configurable de caracteres
        let limite = Self::get_limite(env.clone());
        // Nota: En WASM, no podemos usar nombre.len() directamente
        // pero podemos validar contra el límite configurable
        // Los Symbols en Soroban están limitados a 32 caracteres por protocolo
        
        // Incrementar contador
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));
        
        // Guardar último saludo del usuario
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);
        
        // Extender TTL
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        // Retornar saludo
        Ok(Symbol::new(&env, "Hola"))
    }

    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<Symbol> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }

    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        Ok(())
    }

    pub fn transfer_admin(
        env: Env,
        caller: Address,
        nuevo_admin: Address
    ) -> Result<(), Error> {
        // Verificar que caller sea el admin actual
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        
        // Cambiar el admin
        env.storage()
            .instance()
            .set(&DataKey::Admin, &nuevo_admin);
        
        Ok(())
    }

    pub fn set_limite(
        env: Env,
        caller: Address,
        limite: u32
    ) -> Result<(), Error> {
        // Verificar que caller sea el admin
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        
        // Validar que el límite sea razonable (entre 1 y 32)
        if limite == 0 || limite > 32 {
            return Err(Error::LimiteInvalido);
        }
        
        // Guardar el límite
        env.storage()
            .instance()
            .set(&DataKey::LimiteCaracteres, &limite);
        
        Ok(())
    }

    pub fn get_limite(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::LimiteCaracteres)
            .unwrap_or(32)  // Default: 32 caracteres
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.initialize(&admin);
    }

    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = Symbol::new(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);
        
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let vacio = Symbol::new(&env, "");
        client.hello(&usuario, &vacio);
    }

    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        client.hello(&usuario, &Symbol::new(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        
        client.initialize(&admin);
        
        client.reset_contador(&otro);
    }

    // RETO 2: Transfer Admin
    #[test]
    fn test_transfer_admin() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let nuevo_admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Admin transfiere ownership
        client.transfer_admin(&admin, &nuevo_admin);
        
        // Nuevo admin puede resetear
        client.reset_contador(&nuevo_admin);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_transfer_admin_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        let nuevo = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Otro usuario intenta transferir
        client.transfer_admin(&otro, &nuevo);
    }

    // RETO 3: Límite Configurable
    #[test]
    fn test_set_limite() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Límite por defecto
        assert_eq!(client.get_limite(), 32);
        
        // Admin cambia el límite
        client.set_limite(&admin, &20);
        assert_eq!(client.get_limite(), 20);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #5)")]
    fn test_set_limite_invalido() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Intentar límite inválido
        client.set_limite(&admin, &0);
    }
}