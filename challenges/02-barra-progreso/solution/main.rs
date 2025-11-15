use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🎯 Reto 02: Barra de Progreso\n");
    
    let ancho_barra = 20;
    
    for progreso in 0..=100 {
        // Calcular cuántos bloques llenos
        let bloques_llenos = (progreso * ancho_barra) / 100;
        let bloques_vacios = ancho_barra - bloques_llenos;
        
        // Construir la barra
        let barra_llena = "█".repeat(bloques_llenos);
        let barra_vacia = "░".repeat(bloques_vacios);
        
        // Imprimir en la misma línea usando \r
        print!("\r[{}{}] {}%", barra_llena, barra_vacia, progreso);
        io::stdout().flush().unwrap();
        
        sleep(Duration::from_millis(50));
    }
    
    println!("\n\n✅ ¡Completado!");
}