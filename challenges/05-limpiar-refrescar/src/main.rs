use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    println!("🎯 Reto 05: Limpiar y Refrescar Pantalla\n");
    
    // TODO: Implementa un contador con reloj que se actualiza cada segundo
    // Pistas:
    // 1. Crea un loop infinito
    // 2. Limpia la pantalla en cada iteración
    // 3. Incrementa un contador
    // 4. Obtén la hora actual con SystemTime::now()
    // 5. Formatea y muestra la información
    // 6. Duerme por 1 segundo
    
    let mut contador = 0;
    
    loop {
        // Tu código aquí
        
        contador += 1;
        sleep(Duration::from_secs(1));
    }
}
