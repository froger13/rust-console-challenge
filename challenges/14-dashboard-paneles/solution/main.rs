use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn limpiar_pantalla() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn obtener_timestamp() -> String {
    let ahora = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let horas = (ahora / 3600) % 24;
    let minutos = (ahora / 60) % 60;
    let segundos = ahora % 60;
    
    format!("{:02}:{:02}:{:02}", horas, minutos, segundos)
}

fn generar_valor_aleatorio(seed: u64) -> u64 {
    (seed * 1103515245 + 12345) % 100
}

fn dibujar_barra_progreso(porcentaje: u64, ancho: usize) -> String {
    let lleno = (porcentaje as usize * ancho) / 100;
    let vacio = ancho - lleno;
    format!("[{}{}] {}%", "█".repeat(lleno), "░".repeat(vacio), porcentaje)
}

fn panel_cpu(seed: u64) {
    let cpu_usage = generar_valor_aleatorio(seed) % 100;
    println!("│ 🖥️  CPU Usage          │ 💾 Memory Usage         │");
    println!("│                         │                         │");
    println!("│ {}   │ Used: 5.2GB / 8GB       │", dibujar_barra_progreso(cpu_usage, 10));
    println!("│ Cores: 8                │ {}   │", dibujar_barra_progreso(65, 10));
}

fn panel_procesos() {
    println!("│ 📊 Active Processes     │ 📝 Recent Logs          │");
    println!("│                         │                         │");
    println!("│ nginx      [RUNNING]    │ [{}] Server OK    │", obtener_timestamp());
    println!("│ postgres   [RUNNING]    │ [{}] Request +1   │", obtener_timestamp());
    println!("│ redis      [STOPPED]    │ [{}] Connected    │", obtener_timestamp());
}

fn dibujar_dashboard(iteracion: u64) {
    limpiar_pantalla();
    
    println!("╔═════════════════════════╦═════════════════════════╗");
    println!("║ 📊 DASHBOARD DE SISTEMA ║      {}      ║", obtener_timestamp());
    println!("╠═════════════════════════╩═════════════════════════╣");
    
    // Panel superior
    println!("┌─────────────────────────┬─────────────────────────┐");
    panel_cpu(iteracion);
    println!("└─────────────────────────┴─────────────────────────┘");
    
    // Panel inferior
    println!("┌─────────────────────────┬─────────────────────────┐");
    panel_procesos();
    println!("└─────────────────────────┴─────────────────────────┘");
    
    println!("\nPresiona Ctrl+C para salir");
}

fn main() {
    println!("🎯 Reto 14: Dashboard con Múltiples Paneles\n");
    println!("Iniciando dashboard...\n");
    
    sleep(Duration::from_secs(2));
    
    let mut iteracion = 0;
    
    loop {
        dibujar_dashboard(iteracion);
        iteracion += 1;
        sleep(Duration::from_secs(2));
    }
}
