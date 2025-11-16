# 🎯 Reto 09: Animación de Movimiento Simple

## 📋 Descripción

Crea una animación donde un objeto (como un emoji 🚀 o una letra O) se mueve horizontalmente de izquierda a derecha y rebota en los bordes.

## 🎬 Comportamiento esperado

```
                    🚀







```

El objeto se mueve suavemente de izquierda a derecha, y cuando llega al borde, rebota hacia la izquierda.

## 🎓 Conceptos a aprender

- Animación usando posición y velocidad
- Detección de límites (bordes de la pantalla)
- Rebote (inversión de dirección)
- Refrescar pantalla rápidamente
- Coordenadas en consola

## 💡 Pistas

1. Define variables para posición X y dirección (velocidad)
2. En cada frame, actualiza la posición
3. Si la posición alcanza un borde, invierte la dirección
4. Limpia la pantalla y dibuja el objeto en la nueva posición
5. Usa espacios para posicionar el objeto horizontalmente
6. Ajusta el sleep para controlar la velocidad de animación

## ✅ Criterios de éxito

- [ ] El objeto se mueve suavemente
- [ ] El objeto rebota en ambos bordes
- [ ] La animación es continua
- [ ] No hay parpadeo excesivo
- [ ] La velocidad es apropiada

## 🚀 Bonus

- Haz que el objeto se mueva en diagonal (X e Y)
- Agrega múltiples objetos moviéndose
- Cambia el color del objeto al rebotar
- Deja un rastro desvaneciéndose detrás del objeto

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
