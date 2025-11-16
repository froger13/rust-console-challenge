# 🎯 Reto 12: Menú de Selección Múltiple

## 📋 Descripción

Crea un menú donde se pueden seleccionar múltiples opciones usando checkboxes. El usuario puede navegar, marcar/desmarcar opciones y confirmar la selección final.

## 🎬 Comportamiento esperado

```
===== SELECCIONA TUS OPCIONES =====
> [X] Opción 1: JavaScript
  [ ] Opción 2: Python
  [X] Opción 3: Rust
  [ ] Opción 4: Go
  [ ] Opción 5: C++
====================================

W/S: navegar | Espacio: marcar | Enter: confirmar | Q: cancelar
```

## 🎓 Conceptos a aprender

- Estado múltiple (posición + selecciones)
- Vector de booleanos para tracking
- Toggle de estado
- UI interactiva más compleja
- Formateo condicional

## 💡 Pistas

1. Usa un `Vec<bool>` para trackear qué opciones están seleccionadas
2. Mantén un índice para la opción actual (navegación)
3. Usa espacio para toggle la selección de la opción actual
4. Enter confirma y muestra las opciones seleccionadas
5. Dibuja [X] para seleccionado, [ ] para no seleccionado

## ✅ Criterios de éxito

- [ ] Se pueden seleccionar múltiples opciones
- [ ] La navegación funciona correctamente
- [ ] Espacebar marca/desmarca la opción actual
- [ ] Enter muestra las opciones seleccionadas
- [ ] El estado visual es claro (checkboxes)

## 🚀 Bonus

- Agrega opción "Seleccionar todas" / "Deseleccionar todas"
- Valida que al menos una opción esté seleccionada
- Permite grupos de opciones mutuamente excluyentes
- Guarda las selecciones en un archivo

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
