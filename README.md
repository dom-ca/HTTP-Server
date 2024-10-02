# Operating System Course - ITCR

## Implementación de Servidor HTTP en Rust

**Autores**: Dominic Casares Aguirre y Mariana Viquez Monge
II Semestre 2024

### Documentación

#### Descripción del Diseño del Servidor

El servidor HTTP implementado sigue una arquitectura básica basada en un modelo cliente-servidor y utiliza un diseño concurrente para manejar múltiples solicitudes simultáneamente. A continuación, se describen los principales aspectos del diseño:

- **Arquitectura**: El servidor se inicia en una dirección y puerto específicos, configurados en las constantes `HOST` y `PORT`. Utiliza un `TcpListener` para escuchar conexiones entrantes en este puerto y maneja cada conexión en un hilo separado, utilizando un pool de hilos proporcionado por la biblioteca `threadpool`.

- **Gestión de conexiones concurrentes**: Para permitir la concurrencia, el servidor se apoya en un pool de hilos con un número fijo de hilos (en este caso, 4), que son reutilizados para procesar nuevas conexiones. Esto permite que el servidor maneje múltiples clientes simultáneamente sin bloquear el procesamiento de nuevas solicitudes. Cuando llega una nueva conexión, se envía a uno de los hilos del pool para su procesamiento mediante la función `handle_connection`.

- **Operaciones HTTP**: El servidor soporta las siguientes operaciones HTTP:

  - `GET`: Sirve contenido como páginas estáticas o mensajes personalizados si existe una sesión activa.
  - `POST`: Recibe y procesa datos enviados por el cliente, generalmente en el cuerpo de la solicitud.
  - `PUT`: Similar a POST, pero usado para actualizar recursos.
  - `DELETE`: Elimina un recurso en el servidor y confirma la operación al cliente.
  - `Login` y `Logout`: Manejadas mediante las rutas /login y /logout, respectivamente, para la gestión de sesiones del usuario.

  Cada operación se identifica comparando el contenido inicial de la solicitud HTTP con las rutas conocidas, como `"GET /"`, `"POST /data"`, ..., y luego se delega a la función correspondiente para su manejo (`handle_get_request`, `handle_post_request`, ...).

#### Descripción de la Implementación de la Concurrencia

El servidor utiliza concurrencia a través del patrón **pool de hilos** para manejar múltiples conexiones simultáneamente. Este enfoque es más eficiente que crear un nuevo hilo por cada conexión, ya que los hilos en el pool se reutilizan, reduciendo la sobrecarga del sistema.

- **ThreadPool**: El pool de hilos es proporcionado por la biblioteca `threadpool`. En la inicialización del servidor, se crea un pool con 4 hilos:

  ```rust
  let pool = ThreadPool::new(4);
  ```

  Cada conexión entrante se gestiona mediante `listener.incoming()`, y la conexión se pasa a uno de los hilos disponibles en el pool:

  ```rust
  pool.execute(move || {
      handle_connection(stream, session_store);
  });
  ```

- **Evitar bloqueos y condiciones de carrera**: Dado que las conexiones y las sesiones pueden ser accedidas desde múltiples hilos simultáneamente, se utiliza el patrón `Arc` (Contador de Referencias Atómico) junto con `Mutex` (Exclusión Mutua). Esto permite compartir de manera segura los recursos entre hilos, evitando condiciones de carrera.

  - **Arc**: Se utiliza para compartir el `TcpListener` y la estructura de almacenamiento de sesiones (`session_store`) entre hilos.
  - **Mutex**: Protege el acceso concurrente a la estructura `session_store`, evitando que dos hilos modifiquen las sesiones al mismo tiempo. El `Mutex` asegura que solo un hilo a la vez pueda acceder a los datos protegidos:
    ```rust
    let session_store: Arc<std::sync::Mutex<HashMap<String, Session>>> = Arc::new(std::sync::Mutex::new(HashMap::new()));
    ```

### Manejo de Cookies

El servidor utiliza cookies para gestionar sesiones de usuarios. Las cookies permiten al servidor identificar a los usuarios de manera persistente a través de múltiples solicitudes.

- **Creación de cookies**: Las cookies se crean durante el proceso de inicio de sesión. Cuando un usuario accede a la ruta `/login`, se genera un nuevo `session_id` y se asocia a una nueva sesión en el `session_store`. El servidor luego envía una cabecera HTTP al cliente con la cookie:

  ```rust
  Some(format!("Set-Cookie: session_id={}; HttpOnly", new_session_id)),
  ```

  Esta cookie se almacenará en el navegador del cliente y se enviará automáticamente en las solicitudes futuras.

- **Almacenamiento de sesiones**: Las sesiones se almacenan en un `HashMap` protegido por un `Mutex` para asegurar que los accesos concurrentes a la estructura no generen errores:

  ```rust
  session_store.lock().unwrap().insert(new_session_id.to_string(), session);
  ```

  El `HashMap` contiene pares `clave-valor`, donde la clave es el `session_id` de la cookie y el valor es una instancia de la estructura `Session`.

- **Verificación de cookies**: En cada solicitud entrante, el servidor verifica si el cliente ha enviado una cookie válida. La función `extract_cookie` busca la cabecera `Cookie` en la solicitud HTTP y extrae el `session_id`:

  ```rust
  let cookie = extract_cookie(&request);
  ```

  Luego, la función `get_session_id` revisa si este `session_id` está almacenado en el servidor:

  ```rust
  if session_store.contains_key(session_cookie) {
      Some(session_cookie.to_string())
  } else {
      None
  }
  ```

  Si el `session_id` es válido, el servidor reconoce la sesión del usuario y puede personalizar la respuesta.

- **Eliminación de cookies**: Al cerrar sesión mediante la ruta `/logout`, el servidor elimina la sesión correspondiente del `session_store` y expira la cookie enviando una cabecera con el valor `Max-Age=0`:
  ```rust
  Some("Set-Cookie: session_id=deleted; Max-Age=0".to_string())
  ```

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

  ```rust
  let pool = ThreadPool::new(4);
  ```

- **Puerto**: Si deseas cambiar el puerto del servidor, modifica la constante `PORT`:
  ```rust
  const PORT: &str = "8477";
  ```

#### Consideraciones

El servidor asume que el archivo `index.html` está disponible. Si no existe, el servidor lanzará un error al intentar leerlo.
