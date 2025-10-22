# 🪙 Token BDB (Jenny Token)

Una implementación lista para producción de un token compatible con ERC-20, construido en la plataforma de smart contracts Soroban de Stellar.

## 📋 Tabla de Contenidos

- [Descripción General](#descripción-general)
- [Características](#características)
- [Información del Contrato](#información-del-contrato)
- [Prerequisitos](#prerequisitos)
- [Instalación](#instalación)
- [Construcción del Contrato](#construcción-del-contrato)
- [Testing](#testing)
- [Deployment](#deployment)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Funciones del Contrato](#funciones-del-contrato)
- [Estructura del Proyecto](#estructura-del-proyecto)
- [Desarrollo](#desarrollo)
- [Licencia](#licencia)

## 🎯 Descripción General

Token BDB es un smart contract de token completamente funcional desplegado en la testnet de Stellar. El contrato implementa funcionalidad estándar de tokens incluyendo minting, transferencias y seguimiento de balances con manejo integral de errores y características de seguridad.

**Detalles del Token:**
- **Nombre:** Jenny BDB
- **Símbolo:** JENNY
- **Decimales:** 7
- **Red:** Stellar Testnet
- **Contract ID:** `CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW`

## ✨ Características

- ✅ Implementación compatible con ERC-20
- ✅ Funcionalidad de minting segura (solo admin)
- ✅ Seguimiento y consultas de balances
- ✅ Capacidades de transferencia entre addresses
- ✅ Monitoreo del supply total
- ✅ Manejo integral de errores
- ✅ 7 tests unitarios aprobados
- ✅ Código listo para producción

## 📊 Información del Contrato

| Propiedad | Valor |
|----------|-------|
| **Contract Address** | `CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW` |
| **Wasm Hash** | `3c7587df46eafaacae9739a158cd58fb0b11b0d2c7874803ab79f0dc0101f992` |
| **Red** | Testnet (Test SDF Network) |
| **Desplegado el** | 22 de octubre de 2025 |
| **Explorer** | [Ver en Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW) |
| **Deploy Transaction** | [Ver Transacción](https://stellar.expert/explorer/testnet/tx/7c3f0d11437bfe35844829f6c6fca56a314a137064c68d7a8bc5f357f871b488) |

## 🔧 Prerequisitos

Antes de comenzar, asegúrate de tener instalado lo siguiente:

- **Rust** (última versión stable)
- **Stellar CLI** (`stellar-cli`)
- **soroban-sdk** (incluido en dependencias)
- **Git** para control de versiones

### Instalar Stellar CLI

```bash
# macOS/Linux
cargo install --locked stellar-cli

# Verificar instalación
stellar --version
```

## 📥 Instalación

1. **Clonar el repositorio:**

```bash
git clone https://github.com/JennyT3/Clases-Stellar.git
cd Clases-Stellar/token_bdb
```

2. **Instalar dependencias:**

```bash
cargo build
```

## 🏗️ Construcción del Contrato

Construir el contrato WebAssembly (WASM):

```bash
stellar contract build
```

**Output esperado:**
```
✅ Build Complete
   Wasm File: target/wasm32v1-none/release/token_bdb.wasm
   Wasm Hash: 3c7587df46eafaacae9739a158cd58fb0b11b0d2c7874803ab79f0dc0101f992
   Exported Functions: 8 encontradas
```

## 🧪 Testing

Ejecutar la suite de tests completa:

```bash
cargo test
```

**Todos los tests aprobados:**
- ✅ `test_initialize` - Inicialización del contrato
- ✅ `test_cannot_reinitialize` - Previene doble inicialización
- ✅ `test_initialize_empty_name_fails` - Valida el requerimiento de nombre
- ✅ `test_initialize_empty_symbol_fails` - Valida el requerimiento de símbolo
- ✅ `test_mint_and_balance` - Minting y consultas de balance
- ✅ `test_transfer` - Transferencias de tokens entre cuentas
- ✅ `test_transfer_insufficient_balance` - Manejo de error por balance insuficiente

## 🚀 Deployment

### Paso 1: Crear Identity

```bash
# Generar nueva identity
stellar keys generate jenny --network testnet

# Fondear la cuenta (requerido para deployment)
stellar keys fund jenny --network testnet
```

### Paso 2: Desplegar Contrato

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/token_bdb.wasm \
  --source jenny \
  --network testnet
```

### Paso 3: Inicializar Token

```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- initialize \
  --admin jenny \
  --name "Jenny BDB" \
  --symbol "JENNY" \
  --decimals 7
```

## 💡 Ejemplos de Uso

### Consultar Información del Token

**Obtener nombre del token:**
```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- name
```

**Obtener símbolo del token:**
```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- symbol
```

**Obtener decimales:**
```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- decimals
```

### Mint de Tokens

Mintear 1,000 tokens JENNY (con 7 decimales = 10,000,000,000 unidades base):

```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- mint \
  --to jenny \
  --amount 10000000000
```

### Verificar Balance

```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- balance \
  --id jenny
```

### Transferir Tokens

Transferir 100 tokens JENNY a otro address:

```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- transfer \
  --from jenny \
  --to RECIPIENT_ADDRESS \
  --amount 1000000000
```

### Verificar Supply Total

```bash
stellar contract invoke \
  --id CCNPAOSABDEMBQS7ZBDM6NILFJ2ZQCVNGVS46EFAP3ASAKRF2TUSMAYW \
  --source jenny \
  --network testnet \
  -- total_supply
```

## 📚 Funciones del Contrato

| Función | Parámetros | Descripción | Acceso |
|----------|------------|-------------|--------|
| `initialize` | `admin`, `name`, `symbol`, `decimals` | Inicializa el contrato del token | Público (una vez) |
| `name` | Ninguno | Retorna el nombre del token | Público |
| `symbol` | Ninguno | Retorna el símbolo del token | Público |
| `decimals` | Ninguno | Retorna el número de decimales | Público |
| `mint` | `to`, `amount` | Mintea nuevos tokens a un address | Solo admin |
| `balance` | `id` | Retorna el balance de un address | Público |
| `transfer` | `from`, `to`, `amount` | Transfiere tokens entre addresses | Público |
| `total_supply` | Ninguno | Retorna el supply total de tokens | Público |

## 📁 Estructura del Proyecto

```
token_bdb/
├── Cargo.toml              # Dependencias y metadata del proyecto
├── Cargo.lock              # Versiones bloqueadas de dependencias
├── README.md               # Este archivo
├── .gitignore              # Reglas de Git ignore
├── src/
│   ├── lib.rs              # Lógica principal del contrato y entry points
│   ├── storage.rs          # Keys de storage y gestión de datos
│   ├── errors.rs           # Definiciones de errores personalizados
│   └── test.rs             # Suite de tests completa
├── target/                 # Artefactos del build (gitignored)
│   └── wasm32v1-none/
│       └── release/
│           └── token_bdb.wasm
└── test_snapshots/         # Datos de snapshots de tests
    └── test/
        ├── test_initialize.1.json
        ├── test_cannot_reinitialize.1.json
        ├── test_initialize_empty_name_fails.1.json
        ├── test_initialize_empty_symbol_fails.1.json
        ├── test_mint_and_balance.1.json
        ├── test_transfer.1.json
        └── test_transfer_insufficient_balance.1.json
```

## 🛠️ Desarrollo

### Entendiendo los Decimales

El token usa 7 decimales (estándar de Stellar). Esto significa:
- 1 JENNY = 10,000,000 unidades base
- 100 JENNY = 1,000,000,000 unidades base
- 0.1 JENNY = 1,000,000 unidades base

### Manejo de Errores

El contrato incluye manejo integral de errores para:
- Estado del contrato sin inicializar
- Intentos de re-inicialización
- Nombre o símbolo vacío
- Balance insuficiente para transferencias
- Addresses inválidos

### Agregar Nuevas Características

1. Modificar la lógica del contrato en `src/lib.rs`
2. Agregar tests correspondientes en `src/test.rs`
3. Actualizar storage si es necesario en `src/storage.rs`
4. Agregar nuevos tipos de error en `src/errors.rs`
5. Reconstruir y testear:
```bash
stellar contract build
cargo test
```

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Por favor siéntete libre de enviar un Pull Request.

## 📄 Licencia

Este proyecto es open source y está disponible bajo la Licencia MIT.

## 👩‍💻 Autora

**Jenny Tejedor**
- GitHub: [@JennyT3](https://github.com/JennyT3)
- Repositorio: [Clases-Stellar](https://github.com/JennyT3/Clases-Stellar)

## 🔗 Enlaces Útiles

- [Documentación de Stellar](https://developers.stellar.org/)
- [Documentación de Soroban](https://soroban.stellar.org/)
- [Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet)
- [Guía de Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli)

---

*Construido con ❤️ usando Stellar Soroban*
