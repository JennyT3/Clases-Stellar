## 🎯 Ejercicios Completados

### 📁 Ejercicio 1: Creación Masiva de Cuentas
**Archivo:** `javascript-sdk/crear-cuenta.js`

Genera 5 cuentas Stellar automáticamente usando un bucle `for`, fondea cada una con Friendbot y almacena toda la información en un array.

**Requisitos cumplidos:**
- ✅ Bucle for para generar 5 keypairs
- ✅ Fondeo automático con Friendbot
- ✅ Muestra public key, secret key y balance inicial
- ✅ Almacena información en un array

![Ejercicio 1 - Resultados](capturas/ejercicio1.png)

---

### 💸 Ejercicio 2: Sistema de Pagos Automatizado
**Archivo:** `javascript-sdk/enviar-pago.js`

Sistema que envía 2 XLM a 3 cuentas diferentes con memos únicos ("Tiburoncín Uh Ha Ha 1, 2, 3"), verificando el éxito de cada transacción.

**Requisitos cumplidos:**
- ✅ Envía 2 XLM a 3 destinatarios diferentes
- ✅ Memos únicos para cada transacción
- ✅ Verifica éxito antes de continuar
- ✅ Muestra hash de cada transacción

![Ejercicio 2 - Resultados](capturas/ejercicio2.png)

**🔗 Transacciones en Blockchain:**
- [Pago 1 - Tiburoncín Uh Ha Ha 1](https://stellar.expert/explorer/testnet/tx/d2a7d11b392c1c3170426dabc9117fe81bd43b31431fc3d85ee7d565f04117ddb)
- [Pago 2 - Tiburoncín Uh Ha Ha 2](https://stellar.expert/explorer/testnet/tx/13f05c9bfda2505f20512e8e0d93006bafb879a7aa434363d0fd37d382ad5593)
- [Pago 3 - Tiburoncín Uh Ha Ha 3](https://stellar.expert/explorer/testnet/tx/ca5bbc69a1c20f2fceddfd3264fa233b940d2cb5b3433cdfeae1c7c8c4ff620e)

---

### 📊 Ejercicio 3: Monitor de Balances
**Archivo:** `javascript-sdk/consultar-balance.js`

Monitor que consulta múltiples cuentas mostrando balance de XLM, número de trustlines activos y sequence number con formato legible.

**Requisitos cumplidos:**
- ✅ Acepta array de public keys
- ✅ Muestra balance de XLM
- ✅ Muestra número de trustlines
- ✅ Muestra sequence number
- ✅ Formato de salida legible

![Ejercicio 3 - Resultados](capturas/ejercicio3.png)

---

## 🚀 Instalación y Uso

### Prerrequisitos
- Node.js v18.0 o superior
- npm v9.0 o superior

### Instalación
```bash
# Clonar el repositorio
git clone https://github.com/JennyT3/Clase2-Stellar.git
cd Clase2-Stellar/javascript-sdk

# Instalar dependencias
npm install
Ejecución
bash# Ejercicio 1: Crear 5 cuentas automáticamente
node crear-cuenta.js

# Ejercicio 2: Enviar pagos
# ⚠️ Configurar SECRET_KEY antes de ejecutar
node enviar-pago.js

# Ejercicio 3: Consultar balances
node consultar-balance.js

📦 Tecnologías Utilizadas

Node.js v20.19.0
Stellar SDK v14.2.0
Stellar Testnet


🔒 Seguridad
⚠️ Buenas prácticas implementadas:

Secret keys protegidas con placeholders
Archivos sensibles en .gitignore
Solo uso de Testnet (sin fondos reales)


🎓 Aprendizajes Clave

Automatización de creación de cuentas con bucles
Construcción de transacciones con TransactionBuilder
Manejo de operaciones de pago en Stellar
Consulta de información de cuentas con Horizon API
Uso de memos para identificar transacciones
Buenas prácticas de seguridad en blockchain


📚 Recursos

Stellar Documentation
Stellar SDK para JavaScript
StellarExpert Testnet Explorer
Stellar Laboratory


📄 Estructura del Proyecto
Clase2-Stellar/
├── README.md
├── .gitignore
├── capturas/
│   ├── ejercicio1.png
│   ├── ejercicio2.png
│   └── ejercicio3.png
└── javascript-sdk/
    ├── crear-cuenta.js
    ├── enviar-pago.js
    ├── consultar-balance.js
    ├── package.json
    ├── .gitignore
    └── README.md

🦈 Hecho con dedicación por una Tiburona Builder
Curso Código Futura 2025 - Buen Día Builders
