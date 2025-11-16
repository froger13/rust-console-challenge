use std::io::{self, Write, Read};

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn mostrar_menu(opciones: &[&str], selecciones: &[bool], indice_actual: usize) {
    limpiar_pantalla();
    println!("===== SELECCIONA TUS OPCIONES =====");
    
    for (i, opcion) in opciones.iter().enumerate() {
        let marcador = if i == indice_actual { ">" } else { " " };
        let checkbox = if selecciones[i] { "[X]" } else { "[ ]" };
        println!("{} {} {}", marcador, checkbox, opcion);
    }
    
    println!("====================================\n");
    println!("W/S: navegar | Espacio: marcar | Enter: confirmar | Q: cancelar");
}

fn main() {
    println!("🎯 Reto 12: Menú de Selección Múltiple\n");
    
    let opciones = vec![
        "Opción 1: JavaScript",
        "Opción 2: Python",
        "Opción 3: Rust",
        "Opción 4: Go",
        "Opción 5: C++",
    ];
    
    let mut selecciones = vec![false; opciones.len()];
    let mut indice_actual = 0;
    
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    
    loop {
        mostrar_menu(&opciones, &selecciones, indice_actual);
        
        if stdin.read(&mut buffer).is_ok() {
            match buffer[0] as char {
                'w' | 'W' => {
                    if indice_actual == 0 {
                        indice_actual = opciones.len() - 1;
                    } else {
                        indice_actual -= 1;
                    }
                }
                's' | 'S' => {
                    indice_actual = (indice_actual + 1) % opciones.len();
                }
                ' ' => {
                    // Toggle selección
                    selecciones[indice_actual] = !selecciones[indice_actual];
                }
                '\n' => {
                    // Confirmar selección
                    limpiar_pantalla();
                    println!("Has seleccionado:\n");
                    
                    let mut alguna_seleccionada = false;
                    for (i, &seleccionado) in selecciones.iter().enumerate() {
                        if seleccionado {
                            println!("  ✓ {}", opciones[i]);
                            alguna_seleccionada = true;
                        }
                    }
                    
                    if !alguna_seleccionada {
                        println!("  (Ninguna opción seleccionada)");
                    }
                    
                    println!("\n¡Gracias!");
                    break;
                }
                'q' | 'Q' => {
                    limpiar_pantalla();
                    println!("Operación cancelada.");
                    break;
                }
                _ => {}
            }
        }
    }
}
