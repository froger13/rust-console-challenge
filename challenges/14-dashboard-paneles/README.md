# 🎯 Reto 14: Dashboard con Múltiples Paneles

## 📋 Descripción

Crea un dashboard de monitoreo estilo terminal que muestre múltiples paneles con información actualizada en tiempo real: CPU, memoria, procesos, logs, etc.

## 🎬 Comportamiento esperado

```
┌─────────────────────────┬─────────────────────────┐
│ 🖥️  CPU Usage          │ 💾 Memory Usage         │
│                         │                         │
│ [████████░░] 80%        │ Used: 5.2GB / 8GB       │
│ Cores: 8                │ [██████░░░░] 65%        │
└─────────────────────────┴─────────────────────────┘
┌─────────────────────────┬─────────────────────────┐
│ 📊 Active Processes     │ 📝 Recent Logs          │
│                         │                         │
│ nginx      [RUNNING]    │ [10:34:21] Server OK    │
│ postgres   [RUNNING]    │ [10:34:18] Request +1   │
│ redis      [STOPPED]    │ [10:34:10] Connected    │
└─────────────────────────┴─────────────────────────┘
```

## 🎓 Conceptos a aprender

- Layout en múltiples paneles
- Actualización asíncrona de datos
- Formateo complejo de texto
- Organización de código (módulos/funciones)
- Simulación de métricas del sistema

## 💡 Pistas

1. Divide la pantalla en secciones con bordes
2. Crea funciones para cada panel
3. Usa datos simulados (números aleatorios, timestamps)
4. Actualiza todo el dashboard periódicamente
5. Mantén consistente el tamaño de los paneles

## ✅ Criterios de éxito

- [ ] Al menos 4 paneles diferentes
- [ ] Los paneles tienen bordes claros
- [ ] Los datos se actualizan en tiempo real
- [ ] El layout es consistente
- [ ] La información es legible

## 🚀 Bonus

- Lee datos reales del sistema (si es posible)
- Agrega gráficos de barras o líneas
- Implementa scroll en paneles de logs
- Permite cambiar entre vistas diferentes
- Agrega colores para estados (verde/rojo)

## 📝 Plantilla inicial

El archivo `src/main.rs` contiene una estructura básica. ¡Complétalo!

## 🔍 Solución

Cuando termines tu implementación, revisa `solution/main.rs` para comparar enfoques.
