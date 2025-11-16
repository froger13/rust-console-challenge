# 🎯 Reto 11: Área Delimitada con Colisiones

## 📋 Descripción

Extiende el reto anterior agregando obstáculos (paredes) dentro del área de juego. El jugador no puede atravesar estos obstáculos.

## 🎬 Comportamiento esperado

```
┌──────────────────┐
│ #####            │
│     #            │
│  @  #            │
│     #            │
│ #####      ####  │
└──────────────────┘

W/A/S/D para mover, Q para salir
```

El jugador @ no puede moverse a través de las paredes #.

## 🎓 Conceptos a aprender

- Detección de colisiones
- Representación de mapa con matriz 2D
- Validación de movimientos
- Diseño de niveles básico
- Enum para tipos de celda

## 💡 Pistas

1. Define un enum `Celda` con variantes: Vacio, Pared, Jugador
2. Crea una matriz 2D para representar el mapa
3. Antes de mover al jugador, verifica si la celda destino es Vacio
4. Solo permite el movimiento si no hay colisión
5. Actualiza la matriz después de cada movimiento válido

## ✅ Criterios de éxito

- [ ] El jugador no puede atravesar paredes
- [ ] El mapa tiene varios obstáculos
- [ ] El movimiento solo se permite en celdas vacías
- [ ] La visualización es clara (# para paredes, @ para jugador)
- [ ] El juego responde correctamente a intentos de movimiento inválidos

## 🚀 Bonus

- Carga mapas desde archivos de texto
- Agrega diferentes tipos de obstáculos
- Implementa puertas que se pueden abrir
- Agrega enemigos que se mueven automáticamente

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
