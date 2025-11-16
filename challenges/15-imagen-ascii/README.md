# 🎯 Reto 15: Convertidor de Imagen a ASCII

## 📋 Descripción

Crea un programa que tome datos de píxeles simulados y los convierta en arte ASCII usando diferentes caracteres según la intensidad.

## 🎬 Comportamiento esperado

```
@@@@@@@@@@@@@@@@@@@@
@@@@@@@@##########@@
@@@@@@##          ##
@@@@##              
@@##      ....      
@@##      ....      
@@@@##              
@@@@@@##          ##
@@@@@@@@##########@@
@@@@@@@@@@@@@@@@@@@@
```

Convierte valores de brillo en caracteres ASCII apropiados.

## 🎓 Conceptos a aprender

- Mapeo de valores a caracteres
- Procesamiento de matrices 2D
- Escala de grises a ASCII
- Redimensionamiento de datos
- Arte ASCII programático

## 💡 Pistas

1. Define una escala de caracteres ASCII por densidad: `" .:-=+*#%@"`
2. Para cada píxel, mapea su brillo (0-255) a un carácter
3. Crea una función que tome un valor de brillo y retorne el carácter
4. Procesa la matriz píxel por píxel
5. Para este reto, puedes simular una imagen con un patrón geométrico

## ✅ Criterios de éxito

- [ ] Mapea correctamente brillo a caracteres
- [ ] Funciona con diferentes patrones
- [ ] La salida es visualmente reconocible
- [ ] El código es modular y reutilizable
- [ ] Maneja diferentes tamaños de entrada

## 🚀 Bonus

- Lee archivos de imagen reales (requiere crate externo para el bonus)
- Permite ajustar contraste
- Soporta color ANSI
- Guarda salida en archivo de texto
- Implementa dithering para mejor calidad

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.

## 📌 Nota

Este reto usa solo `std`. Para trabajar con imágenes reales, necesitarías crates como `image`, pero aquí trabajaremos con datos simulados.
