use std::{sync::Arc, time::Duration};

use tokio::{spawn, sync::Semaphore, time::sleep};

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(2));
    let sem_clone = Arc::clone(&sem);

    spawn(async move {
        // Obtenemos un permiso (se le saca uno al semáforo)
        let permit = sem_clone.acquire().await.unwrap();

        // Do some work
        println!("permit_thread: {:?}", permit);
        sleep(Duration::from_secs(10)).await;

        // drop(permit);
    });

    // Obtenemos un permiso (se le saca uno al semáforo)
    let permit_a = sem.acquire().await.unwrap();
    println!("permit_a: {:?}", permit_a);

    // Esperamos un poco
    sleep(Duration::from_secs(2)).await;

    // En este punto no hay permisos disponibles, así que el thread principal se queda esperando
    // al thread que liberará el permiso

    // Obtenemos otro permiso (se le saca uno al semáforo)
    let permit_b = sem.acquire().await.unwrap();
    println!("permit_b: {:?}", permit_b);

    // drop(permit_a);
    // drop(permit_b);
}
