# 🎯 Reto 06: Tabla Simple de Datos

## 📋 Descripción

Crea un programa que muestre una tabla simple con datos de ejemplo (nombres, edades, ciudades) sin bordes decorativos, solo alineación.

## 🎬 Comportamiento esperado

```
NOMBRE          EDAD    CIUDAD
Juan            25      Madrid
María           30      Barcelona
Pedro           22      Valencia
Ana             28      Sevilla
```

## 🎓 Conceptos a aprender

- Alineación de texto con espacios
- Formateo con anchura fija `{:width$}`
- Estructuras de datos (Vec, struct)
- Iteración sobre colecciones
- Padding de strings

## 💡 Pistas

1. Define un struct `Persona` con campos nombre, edad y ciudad
2. Crea un Vec con varias personas de ejemplo
3. Usa `format!("{:<20}", texto)` para alinear a la izquierda con ancho fijo
4. Imprime primero el encabezado, luego itera sobre los datos
5. Mantén consistente el ancho de las columnas

## ✅ Criterios de éxito

- [ ] La tabla tiene encabezados claros
- [ ] Las columnas están alineadas correctamente
- [ ] Se muestran al menos 4 filas de datos
- [ ] El ancho de las columnas es consistente
- [ ] El código usa estructuras de datos apropiadas

## 🚀 Bonus

- Agrega más columnas (email, teléfono)
- Permite ordenar por diferentes campos
- Agrega paginación para muchos datos
- Colorea el encabezado

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
