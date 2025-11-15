use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🎯 Reto 02: Barra de Progreso\n");
    
    let ancho_barra = 20;
    
    // TODO: Implementa la barra de progreso aquí
    // Pistas:
    // 1. Itera de 0 a 100 (porcentaje)
    // 2. Calcula cuántos bloques llenos según el porcentaje
    // 3. Imprime '[' + bloques llenos + bloques vacíos + ']' + porcentaje
    // 4. Usa \r para sobrescribir la misma línea
    // 5. Haz flush después de cada actualización
    
    for progreso in 0..=100 {
        // Tu código aquí
        
        sleep(Duration::from_millis(50));
    }
    
    println!("\n\n✅ ¡Completado!");
}