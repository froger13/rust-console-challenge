# 🎯 Reto 10: Player Controlable (WASD)

## 📋 Descripción

Crea un juego simple donde puedes mover un personaje (emoji o carácter) en un área 2D usando las teclas WASD.

## 🎬 Comportamiento esperado

```
┌──────────────────┐
│                  │
│                  │
│        @         │
│                  │
│                  │
└──────────────────┘

W/A/S/D para mover, Q para salir
```

El personaje @ (o cualquier otro carácter) se mueve por la pantalla según las teclas presionadas.

## 🎓 Conceptos a aprender

- Control en 2D (coordenadas X, Y)
- Input no bloqueante o por caracter
- Actualización de estado basado en input
- Renderizado de área de juego
- Validación de límites

## 💡 Pistas

1. Define posiciones X e Y para el jugador
2. Define el tamaño del área de juego
3. Lee input del usuario (w/a/s/d)
4. Actualiza la posición según el input
5. Valida que no se salga de los límites
6. Redibuja el área con el jugador en la nueva posición

## ✅ Criterios de éxito

- [ ] El jugador puede moverse en las 4 direcciones
- [ ] El jugador no puede salir del área delimitada
- [ ] La pantalla se actualiza correctamente
- [ ] Hay un marco visual que define el área
- [ ] Se puede salir del juego con 'q'

## 🚀 Bonus

- Agrega objetos que el jugador puede recoger
- Implementa puntuación
- Agrega obstáculos que bloquean el movimiento
- Cambia el sprite del jugador según la dirección

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
