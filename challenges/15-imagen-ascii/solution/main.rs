fn brillo_a_ascii(brillo: u8) -> char {
    // Escala de caracteres ASCII ordenados por densidad visual
    let caracteres = " .:-=+*#%@";
    let indice = (brillo as usize * caracteres.len()) / 256;
    caracteres.chars().nth(indice.min(caracteres.len() - 1)).unwrap()
}

fn generar_imagen_circulo(ancho: usize, alto: usize) -> Vec<Vec<u8>> {
    let mut imagen = vec![vec![0u8; ancho]; alto];
    let centro_x = ancho as f32 / 2.0;
    let centro_y = alto as f32 / 2.0;
    let radio = centro_x.min(centro_y) * 0.8;
    
    for y in 0..alto {
        for x in 0..ancho {
            let dx = x as f32 - centro_x;
            let dy = (y as f32 - centro_y) * 2.0; // Ajuste por aspect ratio de caracteres
            let distancia = (dx * dx + dy * dy).sqrt();
            
            // Crear un gradiente circular
            if distancia < radio {
                let intensidad = ((radio - distancia) / radio * 255.0) as u8;
                imagen[y][x] = intensidad;
            } else {
                imagen[y][x] = 0;
            }
        }
    }
    
    imagen
}

fn generar_imagen_gradiente(ancho: usize, alto: usize) -> Vec<Vec<u8>> {
    let mut imagen = vec![vec![0u8; ancho]; alto];
    
    for y in 0..alto {
        for x in 0..ancho {
            // Gradiente diagonal
            let valor = ((x + y) * 255) / (ancho + alto);
            imagen[y][x] = valor as u8;
        }
    }
    
    imagen
}

fn convertir_a_ascii(imagen: &Vec<Vec<u8>>) -> Vec<String> {
    let mut resultado = Vec::new();
    
    for fila in imagen {
        let linea: String = fila.iter().map(|&brillo| brillo_a_ascii(brillo)).collect();
        resultado.push(linea);
    }
    
    resultado
}

fn main() {
    println!("🎯 Reto 15: Convertidor de Imagen a ASCII\n");
    
    println!("Ejemplo 1: Círculo con gradiente\n");
    let imagen_circulo = generar_imagen_circulo(40, 20);
    let ascii_circulo = convertir_a_ascii(&imagen_circulo);
    
    for linea in &ascii_circulo {
        println!("{}", linea);
    }
    
    println!("\n\nEjemplo 2: Gradiente diagonal\n");
    let imagen_gradiente = generar_imagen_gradiente(40, 15);
    let ascii_gradiente = convertir_a_ascii(&imagen_gradiente);
    
    for linea in &ascii_gradiente {
        println!("{}", linea);
    }
    
    println!("\n\nEscala de caracteres usada: \" .:-=+*#%@\"");
    println!("(Del más claro al más oscuro)");
}
