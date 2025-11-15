use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🎯 Reto 03: Loader de Porcentaje\n");
    
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    
    // TODO: Implementa el loader con spinner y porcentaje
    // Pistas:
    // 1. Necesitas dos contadores: uno para spinner, otro para porcentaje
    // 2. El spinner debe rotar más rápido que el porcentaje
    // 3. Usa módulo para ciclar el spinner
    // 4. Imprime en la misma línea con \r
    
    let mut spinner_index = 0;
    
    for progreso in 0..=100 {
        // Tu código aquí
        
        sleep(Duration::from_millis(100));
    }
    
    println!("\n\n✅ ¡Carga completada!");
}