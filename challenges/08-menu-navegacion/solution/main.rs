use std::io::{self, Write, Read};

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn mostrar_menu(opciones: &[&str], seleccion: usize) {
    limpiar_pantalla();
    println!("===== MENÚ PRINCIPAL =====");
    for (i, opcion) in opciones.iter().enumerate() {
        if i == seleccion {
            println!("> {}", opcion);
        } else {
            println!("  {}", opcion);
        }
    }
    println!("===========================\n");
    println!("Usa 'w' (arriba) / 's' (abajo) / 'enter' (seleccionar) / 'q' (salir):");
}

fn main() {
    println!("🎯 Reto 08: Menú con Navegación\n");
    
    let opciones = vec![
        "Opción 1: Iniciar",
        "Opción 2: Configurar",
        "Opción 3: Ayuda",
        "Opción 4: Salir",
    ];
    
    let mut seleccion = 0;
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    
    loop {
        mostrar_menu(&opciones, seleccion);
        
        // Leer un carácter
        if stdin.read(&mut buffer).is_ok() {
            match buffer[0] as char {
                'w' | 'W' => {
                    // Mover arriba (circular)
                    if seleccion == 0 {
                        seleccion = opciones.len() - 1;
                    } else {
                        seleccion -= 1;
                    }
                }
                's' | 'S' => {
                    // Mover abajo (circular)
                    seleccion = (seleccion + 1) % opciones.len();
                }
                '\n' => {
                    // Seleccionar
                    limpiar_pantalla();
                    println!("¡Seleccionaste: {}!\n", opciones[seleccion]);
                    
                    if seleccion == opciones.len() - 1 {
                        println!("Saliendo del programa...");
                        break;
                    }
                    
                    println!("Presiona 'enter' para volver al menú...");
                    let mut dummy = [0u8; 1];
                    stdin.read(&mut dummy).unwrap();
                }
                'q' | 'Q' => {
                    limpiar_pantalla();
                    println!("Saliendo del programa...");
                    break;
                }
                _ => {}
            }
        }
    }
}
