# 🎯 Reto 07: Tabla con Bordes

## 📋 Descripción

Mejora la tabla del reto anterior agregando bordes decorativos usando caracteres ASCII o Unicode.

## 🎬 Comportamiento esperado

```
┌────────────────┬────────┬──────────────┐
│ NOMBRE         │ EDAD   │ CIUDAD       │
├────────────────┼────────┼──────────────┤
│ Juan           │ 25     │ Madrid       │
│ María          │ 30     │ Barcelona    │
│ Pedro          │ 22     │ Valencia     │
│ Ana            │ 28     │ Sevilla      │
└────────────────┴────────┴──────────────┘
```

## 🎓 Conceptos a aprender

- Caracteres Unicode para bordes (┌ ─ ┐ │ ├ ┤ └ ┘ ┬ ┴ ┼)
- Caracteres ASCII alternativos (+, -, |)
- Construcción de strings repetidos
- Separación de lógica de presentación

## 💡 Pistas

1. Define constantes para los caracteres de borde
2. Crea funciones para imprimir línea superior, separador y línea inferior
3. Usa `"─".repeat(ancho)` para crear líneas horizontales
4. Calcula el ancho de cada columna basándote en el contenido
5. Usa padding para centrar o alinear el texto dentro de las celdas

## ✅ Criterios de éxito

- [ ] La tabla tiene bordes en todos los lados
- [ ] El encabezado está separado del contenido
- [ ] Las columnas están correctamente alineadas
- [ ] Los anchos son consistentes
- [ ] El código es reutilizable (funciones)

## 🚀 Bonus

- Permite cambiar entre estilos de bordes (ASCII vs Unicode)
- Agrega colores a los bordes o encabezados
- Soporta texto que contenga caracteres especiales
- Implementa ajuste automático de ancho de columnas

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
