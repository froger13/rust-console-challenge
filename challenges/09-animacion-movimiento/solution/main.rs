use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    println!("🎯 Reto 09: Animación de Movimiento Simple\n");
    println!("Presiona Ctrl+C para salir\n");
    
    sleep(Duration::from_secs(2));
    
    let ancho_pantalla = 60;
    let mut posicion: i32 = 0;
    let mut direccion: i32 = 1; // 1 = derecha, -1 = izquierda
    
    loop {
        limpiar_pantalla();
        
        // Dibujar el objeto en la posición actual
        println!("\n\n");
        let espacios = " ".repeat(posicion as usize);
        println!("{}🚀", espacios);
        
        // Actualizar posición
        posicion += direccion;
        
        // Rebotar en los bordes
        if posicion >= ancho_pantalla {
            posicion = ancho_pantalla;
            direccion = -1;
        } else if posicion <= 0 {
            posicion = 0;
            direccion = 1;
        }
        
        // Pausa para controlar velocidad
        sleep(Duration::from_millis(50));
    }
}
