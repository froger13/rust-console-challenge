# 🎯 Reto 03: Loader de Porcentaje

## 📋 Descripción

Crea un loader circular que muestre un porcentaje de 0% a 100% con un spinner rotatorio.

## 🎬 Comportamiento esperado

```
⠋ Cargando... 23%
```

El spinner debe rotar mientras el porcentaje aumenta:
```
⠙ Cargando... 45%
⠹ Cargando... 67%
⠸ Cargando... 89%
⠼ Cargando... 100%
```

## 🎓 Conceptos a aprender

- Uso de caracteres Unicode especiales (Braille)
- Combinación de animaciones múltiples
- Sincronización de velocidades diferentes
- Arrays y ciclos

## 💡 Pistas

1. Define un array con los caracteres del spinner: `['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']`
2. Usa dos contadores: uno para el spinner y otro para el porcentaje
3. El spinner debe girar más rápido que el incremento del porcentaje
4. Usa `\r` para actualizar la misma línea

## ✅ Criterios de éxito

- [ ] El spinner rota suavemente
- [ ] El porcentaje incrementa de 0 a 100
- [ ] El mensaje se mantiene en la misma línea
- [ ] Al llegar a 100% muestra mensaje de completado

## 🚀 Bonus

- Cambia los caracteres del spinner por otros símbolos
- Haz que el porcentaje incremente de forma aleatoria (simular descarga real)
- Agrega un mensaje descriptivo de lo que se está cargando
- Muestra el tiempo transcurrido

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.