struct Persona {
    nombre: String,
    edad: u32,
    ciudad: String,
}

fn imprimir_linea(inicio: &str, medio: &str, fin: &str, anchos: &[usize]) {
    print!("{}", inicio);
    for (i, &ancho) in anchos.iter().enumerate() {
        print!("{}", "─".repeat(ancho + 2));
        if i < anchos.len() - 1 {
            print!("{}", medio);
        }
    }
    println!("{}", fin);
}

fn imprimir_fila(valores: &[&str], anchos: &[usize]) {
    print!("│");
    for (i, &valor) in valores.iter().enumerate() {
        print!(" {:<width$} │", valor, width = anchos[i]);
    }
    println!();
}

fn main() {
    println!("🎯 Reto 07: Tabla con Bordes\n");
    
    let personas = vec![
        Persona {
            nombre: String::from("Juan"),
            edad: 25,
            ciudad: String::from("Madrid"),
        },
        Persona {
            nombre: String::from("María"),
            edad: 30,
            ciudad: String::from("Barcelona"),
        },
        Persona {
            nombre: String::from("Pedro"),
            edad: 22,
            ciudad: String::from("Valencia"),
        },
        Persona {
            nombre: String::from("Ana"),
            edad: 28,
            ciudad: String::from("Sevilla"),
        },
    ];
    
    let anchos = [14, 6, 12];
    
    // Línea superior
    imprimir_linea("┌", "┬", "┐", &anchos);
    
    // Encabezado
    imprimir_fila(&["NOMBRE", "EDAD", "CIUDAD"], &anchos);
    
    // Separador
    imprimir_linea("├", "┼", "┤", &anchos);
    
    // Datos
    for persona in &personas {
        let edad_str = persona.edad.to_string();
        imprimir_fila(&[&persona.nombre, &edad_str, &persona.ciudad], &anchos);
    }
    
    // Línea inferior
    imprimir_linea("└", "┴", "┘", &anchos);
}
