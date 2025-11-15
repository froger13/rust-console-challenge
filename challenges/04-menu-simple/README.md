# 🎯 Reto 04: Menú con Input Simple

## 📋 Descripción

Crea un menú interactivo que muestre opciones y permita al usuario seleccionar una mediante entrada de texto.

## 🎬 Comportamiento esperado

```
=== MENÚ PRINCIPAL ===
1. Opción 1
2. Opción 2
3. Opción 3
4. Salir

Selecciona una opción: _
```

Al seleccionar una opción:
```
Has seleccionado: Opción 1
```

## 🎓 Conceptos a aprender

- Lectura de input del usuario con `stdin()`
- Manejo de errores con `Result` y `unwrap()`
- Parsing de strings a números
- Bucles con control de flujo (`loop`, `break`)
- Pattern matching con `match`

## 💡 Pistas

1. Usa `io::stdin().read_line(&mut input)` para leer input
2. Limpia el input con `.trim()`
3. Convierte a número con `.parse::<u32>()`
4. Usa `match` para manejar las opciones
5. Usa un `loop` para mantener el menú hasta que el usuario salga

## ✅ Criterios de éxito

- [ ] El menú se muestra correctamente
- [ ] El usuario puede ingresar una opción
- [ ] Las opciones ejecutan diferentes acciones
- [ ] La opción "Salir" termina el programa
- [ ] Maneja inputs inválidos sin crashear

## 🚀 Bonus

- Limpia la pantalla después de cada selección
- Agrega más opciones con funcionalidad real
- Valida que el input sea un número válido
- Agrega colores al menú

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.