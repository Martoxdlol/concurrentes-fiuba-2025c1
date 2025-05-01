use std::{
    sync::{Arc, Condvar},
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    // Condvar para la sincronización
    let cvar = Arc::new(Condvar::new());
    // Mutex con nuestra data que importa
    let mutex = Arc::new(std::sync::Mutex::new(0));

    // Clone para enviar al thread
    let cvar_clone = Arc::clone(&cvar);
    // Clone para enviar al thread
    let mutex_clone: Arc<std::sync::Mutex<u32>> = Arc::clone(&mutex);

    // Adquirir lock del mutex
    let guard = mutex.lock().unwrap();

    // Iniciar el thread
    let _thread = spawn(move || {
        // Esperar 2 segundos
        sleep(Duration::from_secs(2));

        // Adquirir lock del mutex
        // En el thread principal ya estamos esperando a la condvar
        // Esto nos debería dejar adquirir el lock sin problemas
        let mut guard = (mutex_clone).lock().unwrap();

        // Modificar el valor del mutex
        *guard = 17;

        // Notificar a la condvar
        cvar_clone.notify_one();

        // IMPORTANTE: No olvides liberar el mutex
        // Cuando se salga del thread automáticamente se liberará

        // También se puede hacer `drop(guard);` para liberar el mutex. Funciona si se hace antes o después de la notificación
    });

    // Imprimir el valor del mutex actual (debería ser 0)
    println!("g: {}", *guard);

    // Esperar a que el thread termine
    let g = cvar.wait_while(guard, |g| !(*g).eq(&17)).unwrap();

    // Imprimir el valor del mutex después de la espera (debería ser 17)
    println!("g: {}", *g);
}
