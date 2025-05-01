use std::{
    sync::{Arc, Condvar},
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    // Condvar para la sincronización
    let cvar = Arc::new(Condvar::new());
    // Mutex con nuestra data que importa
    let mutex = Arc::new(std::sync::Mutex::new(false));

    // Clone para enviar al thread
    let cvar_clone = Arc::clone(&cvar);
    // Clone para enviar al thread
    let mutex_clone: Arc<std::sync::Mutex<bool>> = Arc::clone(&mutex);

    // Iniciar el thread
    spawn(move || {
        loop {
            // Adquirir lock del mutex
            let guard = mutex_clone.lock().unwrap();
            // Esperar al condvar. Mientras esperamos, el mutex puede ser adquirido por el thread principal
            let mut g = cvar_clone.wait_while(guard, |g| *g).unwrap();

            *g = true;

            println!("Set guard to true");

            sleep(Duration::from_secs(2));

            // Notificar a la condvar
            cvar_clone.notify_one();

            // Al repetir el loop se libera el mutex y lo puede adquirir el thread principal
        }
    });

    loop {
        // Adquirir lock del mutex
        let guard = mutex.lock().unwrap();
        // Esperar al condvar. Mientras esperamos, el mutex puede ser adquirido por el thread
        let mut g = cvar.wait_while(guard, |g| !*g).unwrap();

        *g = false;

        println!("Set guard to false");

        sleep(Duration::from_secs(2));

        // Notificar a la condvar
        cvar.notify_one();

        // Al repetir el loop se libera el mutex y lo puede adquirir el thread
    }
}
