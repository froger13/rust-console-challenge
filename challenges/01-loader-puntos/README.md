# 🎯 Reto 01: Loader de Tres Puntos

## 📋 Descripción

Crea un loader animado que muestre "Cargando" seguido de 1, 2 o 3 puntos que se alternen en un ciclo infinito.

## 🎬 Comportamiento esperado

```
Cargando.
```
(pausa 300ms)
```
Cargando..
```
(pausa 300ms)
```
Cargando...
```
(pausa 300ms)
```
Cargando.
```
(y así sucesivamente...)

## 🎓 Conceptos a aprender

- Uso de `print!()` vs `println!()`
- `std::io::Write` y el método `flush()`
- `std::thread::sleep()` para pausas
- Operador módulo `%` para ciclos
- Secuencias de escape ANSI para limpiar pantalla

## 💡 Pistas

1. Usa `print!()` en lugar de `println!()` para escribir sin salto de línea
2. Llama a `io::stdout().flush().unwrap()` después de `print!()` para forzar la salida inmediata
3. Usa `\x1B[2J\x1B[1;1H` para limpiar la pantalla (ANSI escape code)
4. El operador `%` te ayudará a alternar entre 1, 2 y 3 puntos
5. Usa `Duration::from_millis(300)` para la pausa

## ✅ Criterios de éxito

- [ ] El texto "Cargando" se mantiene fijo
- [ ] Los puntos cambian de 1 a 3 y vuelven a 1
- [ ] Hay una pausa visible entre cada cambio
- [ ] La pantalla se limpia en cada iteración (opcional pero recomendado)
- [ ] El programa corre indefinidamente (Ctrl+C para salir)

## 🚀 Bonus

- Cambia el mensaje de "Cargando" a algo personalizado
- Ajusta la velocidad de animación
- Haz que después de 10 segundos diga "¡Completado!" y termine

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.