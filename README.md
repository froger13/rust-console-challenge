# 🦀 Rust Console Challenges

Aprende a crear aplicaciones de consola desde cero **sin librerías externas** (solo `std`).

## 🎯 Objetivo

Dominar las técnicas fundamentales para desarrollar interfaces de consola en Rust:
- ✅ Crear loaders y animaciones
- ✅ Construir menús interactivos
- ✅ Mostrar tablas de datos
- ✅ Refrescar la pantalla
- ✅ Manejar movimiento y colisiones

## 📚 Estructura de Retos

Los retos están organizados por nivel de dificultad:

### 🟢 Nivel 1: Fundamentos
- **Reto 01**: Loader de tres puntos
- **Reto 02**: Barra de progreso
- **Reto 03**: Loader de porcentaje
- **Reto 04**: Menú con input simple
- **Reto 05**: Limpiar y refrescar pantalla

### 🟡 Nivel 2: Interacción
- **Reto 06**: Tabla simple de datos
- **Reto 07**: Tabla con bordes
- **Reto 08**: Menú con navegación (flechas simuladas)
- **Reto 09**: Animación de movimiento simple
- **Reto 10**: Player controlable (WASD)

### 🔴 Nivel 3: Avanzado
- **Reto 11**: Área delimitada con colisiones
- **Reto 12**: Menú de selección múltiple
- **Reto 13**: Juego simple (Snake básico)
- **Reto 14**: Dashboard con múltiples paneles
- **Reto 15**: Convertidor de imagen a ASCII

## 🚀 Cómo usar este repositorio

1. **Clona el repositorio**:
   ```bash
   git clone https://github.com/froger13/rust-console-challenge.git
   cd rust-console-challenge
   ```

2. **Navega a cada reto**:
   ```bash
   cd challenges/01-loader-puntos
   ```

3. **Lee el README del reto** para entender el objetivo

4. **Completa el código** en `src/main.rs`

5. **Ejecuta tu solución**:
   ```bash
   cargo run
   ```

6. **Compara con la solución** en `solution/main.rs` (¡pero intenta primero!)

## 📖 Conceptos que aprenderás

- **ANSI Escape Codes**: Para limpiar pantalla y mover el cursor
- **Buffering**: Usar `flush()` para actualizar inmediatamente
- **Threading**: `sleep()` para animaciones
- **Input/Output**: Leer desde `stdin` y escribir a `stdout`
- **Strings**: Formateo y manipulación de texto
- **Control de flujo**: Bucles para animaciones y menús

## 🛠️ Requisitos

- Rust instalado (versión 1.70+)
- Terminal con soporte ANSI (la mayoría de terminales modernas)

## 📝 Reglas del juego

1. **Solo usa `std`**: Nada de crates externos (salvo el reto final opcional)
2. **Hazlo funcionar primero**: No te preocupes por código perfecto
3. **Experimenta**: Cambia valores, rompe cosas, aprende
4. **Comparte**: Si creas algo genial, haz un PR

## 🎓 Recursos útiles

- [Rust Book](https://doc.rust-lang.org/book/)
- [ANSI Escape Codes](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)
- [std::io documentation](https://doc.rust-lang.org/std/io/)

## 🤝 Contribuir

¿Tienes ideas para más retos? ¡Abre un issue o PR!

---

**¡Empieza con el Reto 01 y diviértete! 🚀**
