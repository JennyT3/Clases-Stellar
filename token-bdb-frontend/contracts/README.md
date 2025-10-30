# 🪙 Token BDB - Buen Día Builders Token

Token ERC-20 compatible implementado en Soroban (Stellar) como parte del Bootcamp de Buen Día Builders.

## 🎯 Características

✅ Inicialización con administrador  
✅ Minteo de tokens (solo admin)  
✅ Transferencias entre usuarios  
✅ Consulta de balances  
✅ Validaciones de seguridad  
✅ 7 tests unitarios (100% passing)

## 🏗️ Arquitectura
```
token_bdb/
├── Cargo.toml          # Configuración del proyecto
└── src/
    ├── lib.rs          # Contrato principal
    ├── storage.rs      # Estructuras de datos
    ├── errors.rs       # Manejo de errores
    └── test.rs         # Tests unitarios
```

## 🚀 Funciones Principales

### `initialize(admin, name, symbol, decimals)`
Inicializa el token con metadatos y un administrador.

### `mint(to, amount)`
Mintea nuevos tokens (solo admin).

### `transfer(from, to, amount)`
Transfiere tokens entre usuarios.

### `balance(account)`
Consulta el balance de una dirección.

### `total_supply()`
Retorna el supply total del token.

## 🧪 Tests
```bash
cargo test
```

**Resultado:** 7 passed ✅

- `test_initialize` - Inicialización correcta
- `test_mint_and_balance` - Minteo funciona
- `test_transfer` - Transferencias exitosas
- `test_cannot_reinitialize` - Protección contra reinicialización
- `test_initialize_empty_name_fails` - Validación de nombre
- `test_initialize_empty_symbol_fails` - Validación de símbolo
- `test_transfer_insufficient_balance` - Validación de balance

## 📦 Build
```bash
stellar contract build
```

Genera: `target/wasm32-unknown-unknown/release/token_bdb.wasm`

## 🌐 Deploy en Testnet
```bash
# Generar identidad
stellar keys generate alice --network testnet

# Fondear cuenta
curl "https://friendbot.stellar.org?addr=<TU_ADDRESS>"

# Deployar
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/token_bdb.wasm \
  --source alice \
  --network testnet
```

## 👩‍💻 Desarrollado por

**Jenny Tejedor**  
Bootcamp Stellar - Buen Día Builders  
Octubre 2025

## 🔗 Links

- [Repositorio Principal](https://github.com/JennyT3/Clases-Stellar)
- [Soroban Docs](https://soroban.stellar.org/)
- [Buen Día Builders](https://buendiabuilders.com/)

---

🦈 **¡Sigue nadando, Tiburona!** 🦈
