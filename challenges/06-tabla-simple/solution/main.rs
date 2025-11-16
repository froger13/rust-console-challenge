struct Persona {
    nombre: String,
    edad: u32,
    ciudad: String,
}

fn main() {
    println!("🎯 Reto 06: Tabla Simple de Datos\n");
    
    // Datos de ejemplo
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
        Persona {
            nombre: String::from("Carlos"),
            edad: 35,
            ciudad: String::from("Bilbao"),
        },
    ];
    
    // Imprimir encabezado
    println!("{:<16}{:<8}{}", "NOMBRE", "EDAD", "CIUDAD");
    
    // Imprimir datos
    for persona in &personas {
        println!("{:<16}{:<8}{}", persona.nombre, persona.edad, persona.ciudad);
    }
}
