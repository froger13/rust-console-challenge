use std::io::{self, Write};

fn main() {
    println!("🎯 Reto 04: Menú con Input Simple\n");
    
    loop {
        // TODO: Implementa el menú aquí
        // Pistas:
        // 1. Imprime las opciones del menú
        // 2. Lee el input del usuario
        // 3. Convierte el input a número
        // 4. Usa match para manejar cada opción
        // 5. Sal del loop cuando el usuario seleccione "Salir"
        
        println!("\n=== MENÚ PRINCIPAL ===");
        println!("1. Opción 1");
        println!("2. Opción 2");
        println!("3. Opción 3");
        println!("4. Salir");
        print!("\nSelecciona una opción: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        // Tu código aquí
        
        break; // Temporal - elimina esto cuando implementes el loop real
    }
}