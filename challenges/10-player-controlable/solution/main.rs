use std::io::{self, Write, Read};

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn dibujar_area(ancho: usize, alto: usize, jugador_x: usize, jugador_y: usize) {
    // Borde superior
    print!("┌");
    for _ in 0..ancho {
        print!("─");
    }
    println!("┐");
    
    // Área de juego
    for y in 0..alto {
        print!("│");
        for x in 0..ancho {
            if x == jugador_x && y == jugador_y {
                print!("@");
            } else {
                print!(" ");
            }
        }
        println!("│");
    }
    
    // Borde inferior
    print!("└");
    for _ in 0..ancho {
        print!("─");
    }
    println!("┘");
}

fn main() {
    println!("🎯 Reto 10: Player Controlable (WASD)\n");
    
    let ancho = 20;
    let alto = 10;
    let mut jugador_x = ancho / 2;
    let mut jugador_y = alto / 2;
    
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    
    loop {
        limpiar_pantalla();
        dibujar_area(ancho, alto, jugador_x, jugador_y);
        println!("\nW/A/S/D para mover, Q para salir");
        
        // Leer input
        if stdin.read(&mut buffer).is_ok() {
            match buffer[0] as char {
                'w' | 'W' => {
                    if jugador_y > 0 {
                        jugador_y -= 1;
                    }
                }
                's' | 'S' => {
                    if jugador_y < alto - 1 {
                        jugador_y += 1;
                    }
                }
                'a' | 'A' => {
                    if jugador_x > 0 {
                        jugador_x -= 1;
                    }
                }
                'd' | 'D' => {
                    if jugador_x < ancho - 1 {
                        jugador_x += 1;
                    }
                }
                'q' | 'Q' => {
                    limpiar_pantalla();
                    println!("¡Gracias por jugar!");
                    break;
                }
                _ => {}
            }
        }
    }
}
