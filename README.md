# 🦈 Clases Stellar - Código Futura

**Estudiante:** Jenny Tejedor  
**Programa:** Bootcamp Stellar de Buen día Builders
**Período:** Octubre 2025

---

## 📚 Contenido del Repositorio

Repositorio con las tareas del Bootcamp Stellar, enfocado en desarrollo de contratos inteligentes con Soroban.

### 🗂️ Estructura
```
Clases-Stellar/
├── javascript-sdk/           # Tarea 1: Stellar SDK (JS)
└── hello-tiburona/           # Tarea 2: Contrato Soroban profesional
```

---

## 📋 Tareas Completadas

### ✅ Tarea 1: JavaScript SDK
**Carpeta:** `javascript-sdk/`

Ejercicios fundamentales con Stellar usando JavaScript SDK.
- Creación de cuentas
- Envío de pagos
- Consulta de balances

[Ver detalles →](./javascript-sdk/README.md)

---


### ✅ Tarea Clase 4: Implementa Hello Tiburona Profesional
**Carpeta:** `hello-tiburona/`

Contrato inteligente en Soroban con arquitectura profesional.

**Características:**
- 🔐 Control de acceso (admin)
- ✅ Validaciones de input
- 📊 Contador de saludos
- 💾 Storage organizado (Instance + Persistent)
- ⏰ TTL management
- 🎯 4 tipos de errores personalizados

**Testing:**
```bash
cargo test
# Result: 6 passed; 0 failed ✅
```

**Build:**
```bash
cargo build --target wasm32-unknown-unknown --release
# WASM: 6.1KB
```

**Funciones principales:**
- `initialize()` - Setup inicial con admin
- `hello()` - Registrar saludo con validaciones
- `get_contador()` - Consultar total de saludos
- `get_ultimo_saludo()` - Ver último saludo de usuario
- `reset_contador()` - Resetear (solo admin)

[Ver código completo →](./hello-tiburona/contracts/hello-tiburona/src/lib.rs)

---

## 🛠️ Stack Tecnológico

- **Rust** (v1.75+) - Lenguaje para Soroban
- **Soroban SDK** (v21.7.7) - Framework de contratos
- **Stellar CLI** (v23.1.4) - Tooling y deployment
- **JavaScript/Node.js** - Ejercicios SDK

---

## 🚀 Quick Start

### Prerequisitos
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Soroban CLI
cargo install --locked soroban-cli

# Target WASM
rustup target add wasm32-unknown-unknown
```

### Ejecutar Hello Tiburona
```bash
cd hello-tiburona
cargo test                    # Tests
cargo build --target wasm32-unknown-unknown --release  # Build
```

---

## 📚 Recursos

- [Stellar Docs](https://developers.stellar.org/)
- [Soroban Docs](https://soroban.stellar.org/)
- [Codigo Futura Repo](https://github.com/BuenDia-Builders/codigofutura)

---

## 📬 Contacto

**Jenny Tejedor**  
GitHub: [@JennyT3](https://github.com/JennyT3)

---

🦈 **Bootcamp Tiburones Stellar** 🦈
