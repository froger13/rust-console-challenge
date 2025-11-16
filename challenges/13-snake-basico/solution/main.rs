use std::collections::VecDeque;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, PartialEq)]
enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn generar_comida(ancho: usize, alto: usize, serpiente: &VecDeque<(usize, usize)>) -> (usize, usize) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    
    loop {
        let x = (timestamp as usize * 13 + serpiente.len()) % ancho;
        let y = (timestamp as usize * 17 + serpiente.len() * 3) % alto;
        
        if !serpiente.contains(&(x, y)) {
            return (x, y);
        }
    }
}

fn dibujar_juego(serpiente: &VecDeque<(usize, usize)>, comida: (usize, usize), ancho: usize, alto: usize) {
    // Borde superior
    print!("┌");
    for _ in 0..ancho {
        print!("─");
    }
    println!("┐");
    
    for y in 0..alto {
        print!("│");
        for x in 0..ancho {
            if serpiente.front() == Some(&(x, y)) {
                print!("○"); // Cabeza
            } else if serpiente.contains(&(x, y)) {
                print!("●"); // Cuerpo
            } else if comida == (x, y) {
                print!("*"); // Comida
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
    println!("🎯 Reto 13: Juego Simple (Snake Básico)\n");
    println!("Nota: Este es un ejemplo simplificado sin input no bloqueante.");
    println!("La serpiente se moverá automáticamente.\n");
    println!("Presiona Ctrl+C para salir\n");
    
    sleep(Duration::from_secs(3));
    
    let ancho = 20;
    let alto = 10;
    
    let mut serpiente = VecDeque::new();
    serpiente.push_back((5, 5));
    serpiente.push_back((4, 5));
    serpiente.push_back((3, 5));
    
    let mut direccion = Direccion::Derecha;
    let mut comida = generar_comida(ancho, alto, &serpiente);
    let mut puntuacion = 0;
    
    loop {
        limpiar_pantalla();
        dibujar_juego(&serpiente, comida, ancho, alto);
        println!("\nPuntuación: {}", puntuacion);
        println!("(Movimiento automático en este ejemplo)");
        
        // Calcular nueva posición de la cabeza
        let (cabeza_x, cabeza_y) = *serpiente.front().unwrap();
        let (nueva_x, nueva_y) = match direccion {
            Direccion::Arriba => (cabeza_x, cabeza_y.wrapping_sub(1)),
            Direccion::Abajo => (cabeza_x, cabeza_y + 1),
            Direccion::Izquierda => (cabeza_x.wrapping_sub(1), cabeza_y),
            Direccion::Derecha => (cabeza_x + 1, cabeza_y),
        };
        
        // Verificar colisión con bordes
        if nueva_x >= ancho || nueva_y >= alto {
            limpiar_pantalla();
            println!("¡GAME OVER! Chocaste con la pared.");
            println!("Puntuación final: {}", puntuacion);
            break;
        }
        
        // Verificar colisión con cuerpo
        if serpiente.contains(&(nueva_x, nueva_y)) {
            limpiar_pantalla();
            println!("¡GAME OVER! Chocaste contigo mismo.");
            println!("Puntuación final: {}", puntuacion);
            break;
        }
        
        // Mover serpiente
        serpiente.push_front((nueva_x, nueva_y));
        
        // Verificar si comió
        if (nueva_x, nueva_y) == comida {
            puntuacion += 1;
            comida = generar_comida(ancho, alto, &serpiente);
        } else {
            serpiente.pop_back();
        }
        
        // Cambio de dirección automático para demo (opcional)
        // En una implementación real, leerías input del usuario aquí
        
        sleep(Duration::from_millis(200));
    }
}
