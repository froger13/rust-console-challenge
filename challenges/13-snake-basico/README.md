# 🎯 Reto 13: Juego Simple (Snake Básico)

## 📋 Descripción

Implementa un juego clásico de Snake simplificado. La serpiente crece al comer comida y el juego termina si choca con los bordes o consigo misma.

## 🎬 Comportamiento esperado

```
┌────────────────────┐
│                    │
│   ●●●○             │
│       *            │
│                    │
└────────────────────┘

Puntuación: 3
W/A/S/D para mover | Q para salir
```

La serpiente (●) se mueve continuamente, come comida (*), y crece.

## 🎓 Conceptos a aprender

- Game loop básico
- Estado de juego complejo
- Manejo de colas (deque)
- Generación de posiciones aleatorias
- Detección de colisiones múltiples
- Lógica de crecimiento

## 💡 Pistas

1. Usa `VecDeque<(usize, usize)>` para la serpiente (cola para eficiencia)
2. La cabeza es el primer elemento, la cola el último
3. Cada frame: agrega nueva cabeza en dirección actual, quita cola (si no comió)
4. Genera comida en posición aleatoria que no sea serpiente
5. Detecta colisión con paredes o cuerpo de la serpiente

## ✅ Criterios de éxito

- [ ] La serpiente se mueve continuamente
- [ ] Puede cambiar dirección con WASD
- [ ] Crece al comer comida
- [ ] Nueva comida aparece después de comer
- [ ] El juego termina en colisión
- [ ] Muestra puntuación

## 🚀 Bonus

- Previene movimiento en dirección opuesta directa
- Aumenta velocidad progresivamente
- Agrega power-ups especiales
- Implementa high scores
- Agrega niveles con obstáculos

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
