# 🎯 Reto 08: Menú con Navegación (Flechas Simuladas)

## 📋 Descripción

Crea un menú interactivo donde se pueda navegar entre opciones usando números para simular las teclas de flecha arriba/abajo.

## 🎬 Comportamiento esperado

```
===== MENÚ PRINCIPAL =====
> Opción 1: Iniciar
  Opción 2: Configurar
  Opción 3: Ayuda
  Opción 4: Salir
===========================

Usa 'w' (arriba) / 's' (abajo) / 'enter' (seleccionar):
```

La opción actual tiene el marcador `>` y al presionar enter ejecuta la acción.

## 🎓 Conceptos a aprender

- Mantener estado de selección
- Leer input de usuario caracter por caracter
- Actualizar pantalla en respuesta a input
- Navegación circular en listas
- Pattern matching en Rust

## 💡 Pistas

1. Mantén un índice `seleccion_actual` que empieza en 0
2. Usa un loop que lee input del usuario
3. Con 'w' decrementa el índice (con wrap-around)
4. Con 's' incrementa el índice (con wrap-around)
5. Con enter, ejecuta la acción de la opción seleccionada
6. Limpia y redibuja el menú en cada cambio

## ✅ Criterios de éxito

- [ ] El menú muestra al menos 4 opciones
- [ ] La navegación con w/s funciona correctamente
- [ ] El marcador `>` indica la opción actual
- [ ] La navegación es circular (después de la última vuelve a la primera)
- [ ] Al presionar enter se ejecuta la acción correspondiente

## 🚀 Bonus

- Agrega colores a la opción seleccionada
- Implementa submenús
- Usa 'q' para salir sin seleccionar
- Agrega descripciones a cada opción

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
