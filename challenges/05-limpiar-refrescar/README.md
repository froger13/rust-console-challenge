# 🎯 Reto 05: Limpiar y Refrescar Pantalla

## 📋 Descripción

Crea un programa que muestre un contador que se incrementa cada segundo, limpiando la pantalla en cada actualización. Adicionalmente, muestra la fecha y hora actual.

## 🎬 Comportamiento esperado

```
===================
  Contador: 5
  Hora: 10:23:45
===================
```

(la pantalla se limpia y actualiza cada segundo)

## 🎓 Conceptos a aprender

- Códigos ANSI para limpiar pantalla
- Obtener fecha y hora con `std::time::SystemTime`
- Formateo de tiempo
- Bucles infinitos con actualizaciones periódicas
- Centrado de texto

## 💡 Pistas

1. Usa `\x1B[2J\x1B[1;1H` para limpiar pantalla y mover cursor al inicio
2. Usa `SystemTime::now()` para obtener el tiempo actual
3. Convierte el tiempo a un formato legible con `chrono` o manualmente
4. Incrementa un contador en cada iteración
5. Actualiza cada segundo con `sleep(Duration::from_secs(1))`

## ✅ Criterios de éxito

- [ ] La pantalla se limpia completamente en cada actualización
- [ ] El contador se incrementa correctamente
- [ ] La hora se actualiza cada segundo
- [ ] El formato es limpio y legible
- [ ] El programa corre indefinidamente

## 🚀 Bonus

- Agrega un marco decorativo alrededor del contenido
- Muestra información adicional (fecha, día de la semana)
- Cambia colores usando códigos ANSI
- Agrega un mensaje personalizado

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
