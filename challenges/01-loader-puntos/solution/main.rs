use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn limpiar_pantalla() {
    // Código ANSI para limpiar pantalla y mover cursor al inicio
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    println!("🎯 Reto 01: Loader de Tres Puntos\n");
    println!("Presiona Ctrl+C para salir\n");
    
    sleep(Duration::from_secs(2));
    
    let mut contador = 0;
    
    loop {
        limpiar_pantalla();
        
        // Calcular número de puntos (1, 2, o 3)
        let num_puntos = (contador % 3) + 1;
        
        // Imprimir "Cargando" con los puntos
        print!("Cargando");
        for _ in 0..num_puntos {
            print!(".");
        }
        
        // Forzar salida inmediata
        io::stdout().flush().unwrap();
        
        // Incrementar contador
        contador += 1;
        
        // Pausa de 300ms
        sleep(Duration::from_millis(300));
    }
}