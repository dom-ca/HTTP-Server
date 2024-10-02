# Operating System Course - ITCR

## Implementacion de Servidor HTTP en Rust

### Documentacion

//// Poner aqui

### Manual de Usuario

#### Introducción

Este manual de usuario explica nuestro código de un servidor HTTP básico escrito en Rust. Puede manejar múltiples solicitudes de clientes mediante hilos, procesar sesiones simples utilizando cookies, y gestionar peticiones HTTP comunes como `GET`, `POST`, `PUT`, `DELETE`, así como solicitudes de login y logout.

#### Requerimientos Previos

Para ejecutar el servidor, necesitarás tener instalado:

- Rust (https://www.rust-lang.org/tools/install).
- **Cargo** para la gestión de dependencias.

Además, el código utiliza las siguientes dependenecias de terceros:

- `threadpool`: para la gestión de un pool de hilos.
- `std::sync::{Arc, Mutex}`: para compartir datos entre hilos de manera segura.

#### Instalación

1. Descarga los archivos de este proyecto.
2. Colócalos en una carpeta donde puedas ejecutarlos.

#### Ejecución del Servidor

1. Posicionate en el directorio del proyecto:

```
cd HTTPServer
```

2. Compila y ejecuta el servidor:

```
cargo run
```

3. El servidor estará escuchando en el puerto **8477** en `127.0.0.1`. Verás en la consola:

```
Web server is listening at port 8477
```

4. Si quieres otra forma de verificar que el servidor está corriendo correctamente, puedes acceder a este link (http://127.0.0.1:8477/)

#### Personalización

- **Número de hilos**: Puedes ajustar el número de hilos en el pool modificando este valor:

```
let pool = ThreadPool::new(4);
```

- **Puerto**: Si deseas cambiar el puerto del servidor, modifica la constante `PORT`:

```
const PORT: &str = "8477";
```

#### Consideraciones

El servidor asume que el archivo `index.html` está disponible. Si no existe, el servidor lanzará un error al intentar leerlo.
