# 🦈 Hello Tiburona - Tarea Clase 4

Contrato inteligente profesional en Soroban (Stellar) implementando un sistema de saludos con control de acceso.

## ✨ Características Implementadas

✅ Manejo de errores con `contracterror` (4 tipos)
✅ Storage organizado con DataKey (Instance + Persistent)
✅ Control de acceso basado en admin
✅ Validaciones de input (nombre vacío y longitud)
✅ Sistema de contador global
✅ Tracking de último saludo por usuario
✅ TTL management correcto
✅ 6 tests comprehensivos (100% passing)

## 🏗️ Arquitectura

### Errores Personalizados
- `NombreVacio` (#1) - Cuando el nombre está vacío
- `NombreMuyLargo` (#2) - Cuando el nombre excede 32 caracteres
- `NoAutorizado` (#3) - Acceso denegado a funciones admin
- `NoInicializado` (#4) - Contrato ya inicializado

### Storage
- **Instance Storage:** Admin, ContadorSaludos
- **Persistent Storage:** UltimoSaludo(Address)

## 🚀 Funciones Públicas

### `initialize(admin: Address) -> Result<(), Error>`
Inicializa el contrato con un administrador. Solo puede llamarse una vez.

### `hello(usuario: Address, nombre: Symbol) -> Result<Symbol, Error>`
Registra un saludo. Valida el nombre y actualiza estadísticas.

### `get_contador() -> u32`
Retorna el número total de saludos registrados.

### `get_ultimo_saludo(usuario: Address) -> Option<Symbol>`
Consulta el último saludo de un usuario específico.

### `reset_contador(caller: Address) -> Result<(), Error>`
Resetea el contador a 0. Solo el admin puede ejecutar esta función.

## 🧪 Tests
```bash
cargo test
```

**Resultado:** 6 passed; 0 failed

- ✅ `test_initialize` - Inicialización exitosa
- ✅ `test_no_reinicializar` - Protección contra reinicialización
- ✅ `test_hello_exitoso` - Función principal funciona correctamente
- ✅ `test_nombre_vacio` - Validación de nombre vacío
- ✅ `test_reset_solo_admin` - Admin puede resetear
- ✅ `test_reset_no_autorizado` - No-admin no puede resetear

## 🏗️ Build
```bash
soroban contract build
```

**Output:**
- WASM generado: `hello_tiburona.wasm`
- 6 funciones exportadas
- Hash: `e8b34e73590f135c15be4e0019169ed5f528c7837061dabcb692f6821504a854`

## 📚 Conceptos Aplicados

- ✅ Result<T, E> para manejo de errores
- ✅ Option<T> para valores opcionales
- ✅ Storage types (Instance vs Persistent)
- ✅ TTL management
- ✅ Control de acceso con Address
- ✅ Validaciones pre-storage
- ✅ Testing comprehensivo

## 👩‍💻 Desarrollado por

**Jenny Tejedor**  
Bootcamp Stellar - Clase 4  
Octubre 2025

---

🦈⚡ **¡Código que funciona, código que importa!** ⚡🦈
