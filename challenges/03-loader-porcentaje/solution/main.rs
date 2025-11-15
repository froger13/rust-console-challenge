use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🎯 Reto 03: Loader de Porcentaje\n");
    
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut spinner_index = 0;
    
    for progreso in 0..=100 {
        // Obtener el carácter actual del spinner
        let spinner = spinner_chars[spinner_index % spinner_chars.len()];
        
        // Imprimir spinner + mensaje + porcentaje
        print!("\r{} Cargando... {}%", spinner, progreso);
        io::stdout().flush().unwrap();
        
        // Incrementar índice del spinner
        spinner_index += 1;
        
        sleep(Duration::from_millis(100));
    }
    
    println!("\n\n✅ ¡Carga completada!");
}