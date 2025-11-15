# 🎯 Reto 02: Barra de Progreso

## 📋 Descripción

Crea una barra de progreso que se llene de 0% a 100% usando caracteres sólidos.

## 🎬 Comportamiento esperado

```
[████████░░░░░░░░░░░░] 40%
```

La barra debe avanzar gradualmente hasta llegar a:

```
[████████████████████] 100%
```

## 🎓 Conceptos a aprender

- Cálculo de proporciones (porcentaje a caracteres)
- Repetición de strings con `repeat()`
- Formateo de strings
- Actualización de una misma línea

## 💡 Pistas

1. Define el ancho total de la barra (por ejemplo, 20 caracteres)
2. Calcula cuántos bloques llenos (`█`) según el porcentaje
3. Los bloques restantes serán vacíos (`░`)
4. Usa `\r` (retorno de carro) para sobrescribir la misma línea
5. No olvides hacer `flush()` después de cada actualización

## ✅ Criterios de éxito

- [ ] La barra tiene un ancho fijo y consistente
- [ ] El porcentaje se calcula correctamente
- [ ] La animación es suave (incrementos pequeños)
- [ ] Se muestra el porcentaje numérico junto a la barra
- [ ] Al llegar a 100% el programa termina o muestra mensaje de completado

## 🚀 Bonus

- Agrega colores usando códigos ANSI
- Muestra tiempo estimado restante
- Haz una barra vertical en lugar de horizontal
- Agrega una etiqueta descriptiva (ej: "Descargando archivo...")

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.