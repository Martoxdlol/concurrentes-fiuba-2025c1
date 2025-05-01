use std::{sync::Arc, time::Duration};

use tokio::{spawn, sync::Semaphore, time::sleep};

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(1));
    let sem_clone = Arc::clone(&sem);

    spawn(async move {
        // Obtenemos un permiso (se le saca uno al semáforo, queda en 0)
        let permit_a = sem_clone.acquire().await.unwrap();

        // Do some work
        println!("permit_a: {:?}", permit_a);
        sleep(Duration::from_secs(10)).await;

        // Acá se libera porque se termina el scope, es lo mismo que hacer `drop(permit);`
    });

    // Esperamos un poco
    sleep(Duration::from_secs(1)).await;

    // Obtenemos otro permiso. Hay que esperar a que el otro thread lo libere
    let permit_b = sem.acquire().await.unwrap();
    println!("permit_b: {:?}", permit_b);
}
