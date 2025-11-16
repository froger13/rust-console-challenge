use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn obtener_hora_formateada() -> String {
    let ahora = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let horas = (ahora / 3600) % 24;
    let minutos = (ahora / 60) % 60;
    let segundos = ahora % 60;
    
    format!("{:02}:{:02}:{:02}", horas, minutos, segundos)
}

fn main() {
    println!("🎯 Reto 05: Limpiar y Refrescar Pantalla\n");
    println!("Presiona Ctrl+C para salir\n");
    
    sleep(Duration::from_secs(2));
    
    let mut contador = 0;
    
    loop {
        limpiar_pantalla();
        
        println!("===================");
        println!("  Contador: {}", contador);
        println!("  Hora: {}", obtener_hora_formateada());
        println!("===================");
        
        contador += 1;
        sleep(Duration::from_secs(1));
    }
}
