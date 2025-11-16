use std::io::{self, Write, Read};

#[derive(Clone, Copy, PartialEq)]
enum Celda {
    Vacio,
    Pared,
}

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn crear_mapa() -> Vec<Vec<Celda>> {
    let ancho = 20;
    let alto = 10;
    let mut mapa = vec![vec![Celda::Vacio; ancho]; alto];
    
    // Crear algunos obstáculos
    // Pared vertical izquierda
    for i in 0..5 {
        mapa[i][1] = Celda::Pared;
    }
    for i in 0..5 {
        mapa[i][5] = Celda::Pared;
    }
    
    // Pared horizontal inferior
    for i in 5..9 {
        mapa[4][i] = Celda::Pared;
    }
    
    // Obstáculo derecha
    for i in 5..9 {
        mapa[i][15] = Celda::Pared;
    }
    
    mapa
}

fn dibujar_mapa(mapa: &Vec<Vec<Celda>>, jugador_x: usize, jugador_y: usize) {
    let ancho = mapa[0].len();
    let alto = mapa.len();
    
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
            } else if mapa[y][x] == Celda::Pared {
                print!("#");
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
    println!("🎯 Reto 11: Área Delimitada con Colisiones\n");
    
    let mapa = crear_mapa();
    let ancho = mapa[0].len();
    let alto = mapa.len();
    
    let mut jugador_x = 3;
    let mut jugador_y = 2;
    
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    
    loop {
        limpiar_pantalla();
        dibujar_mapa(&mapa, jugador_x, jugador_y);
        println!("\nW/A/S/D para mover, Q para salir");
        
        if stdin.read(&mut buffer).is_ok() {
            let (mut nuevo_x, mut nuevo_y) = (jugador_x, jugador_y);
            
            match buffer[0] as char {
                'w' | 'W' => {
                    if nuevo_y > 0 {
                        nuevo_y -= 1;
                    }
                }
                's' | 'S' => {
                    if nuevo_y < alto - 1 {
                        nuevo_y += 1;
                    }
                }
                'a' | 'A' => {
                    if nuevo_x > 0 {
                        nuevo_x -= 1;
                    }
                }
                'd' | 'D' => {
                    if nuevo_x < ancho - 1 {
                        nuevo_x += 1;
                    }
                }
                'q' | 'Q' => {
                    limpiar_pantalla();
                    println!("¡Gracias por jugar!");
                    break;
                }
                _ => {}
            }
            
            // Verificar colisión antes de mover
            if mapa[nuevo_y][nuevo_x] == Celda::Vacio {
                jugador_x = nuevo_x;
                jugador_y = nuevo_y;
            }
        }
    }
}
