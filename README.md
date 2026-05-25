# Proyecto Final: Estructuras de Datos de Grafos y su Implementación en Rust

Este repositorio contiene la estructura base modular para el desarrollo de la **Fase 2 (Evaluación Práctica)** y la documentación de la **Fase 1 (Reporte de Investigación)** del curso de Estructuras de Datos.

---

## 📝 1. Reporte de Investigación (Fase 1) y Diapositivas (Fase 3)

La documentación y material de soporte de este proyecto se encuentra totalmente integrada en plataformas en la nube para asegurar el formato óptimo del evaluador:

👉 **[Reporte Oficial de Investigación - Grafos en Rust (Google Docs)](https://docs.google.com/document/d/1y0whwA74DEcmVAUtZH9WT7GXkOXi2EI3AlAHn7EhiTs/edit?usp=sharing)**

👉 **[Diapositivas Oficiales de Exposición - Grafos en Rust (Google Slides)](https://docs.google.com/presentation/d/1up1nIGgDM09itQDEPvpoJafNgaUPoa2xNQguiJIwc8U/edit?usp=sharing)**

---

## 📁 2. Estructura Modular del Proyecto en Rust

El proyecto está diseñado bajo una arquitectura modular desacoplada en `src/` para facilitar el trabajo en paralelo de los integrantes del equipo:

*   `src/main.rs`: Orquestador principal y punto de entrada.
*   `src/model.rs`: Módulo de definición del dominio real (ciudades, redes viales, etc.).
*   `src/graph_manual.rs`: Estructura de grafo manual basada en vectores contiguos e índices.
*   `src/graph_petgraph.rs`: Estructura de grafo basada en la biblioteca industrial `petgraph`.
*   `src/algorithms.rs`: Implementación de algoritmos de recorrido (BFS/DFS) desacoplados.

---

## 🚀 3. Guía de Trabajo Colaborativo para el Equipo (Paso a Paso)

Para asegurar un desarrollo ordenado y limpio, cada integrante trabajará en su propia rama dedicada. Sigan estas instrucciones paso a paso para evitar conflictos de código:

### 👣 Paso 1: Clonar el Repositorio y Entrar al Proyecto
Abre tu terminal y ejecuta los siguientes comandos:
```bash
git clone https://github.com/Fernando-rtx/UltimaEvalucionESD2026.git
cd UltimaEvalucionESD2026
```

### 👣 Paso 2: Cambiar a tu Rama de Trabajo Asignada
Cada miembro del equipo tiene una rama asignada en el repositorio remoto. Cámbiate a ella antes de programar:

*   **Luis** (Dominio y Modelo):
    ```bash
    git checkout feature/model-luis
    ```
*   **Mario** (Grafo Manual):
    ```bash
    git checkout feature/graph-manual-mario
    ```
*   **Alberto** (Grafo petgraph):
    ```bash
    git checkout feature/graph-petgraph-alberto
    ```
*   **Rodrigo** (Algoritmos BFS/DFS):
    ```bash
    git checkout feature/algorithms-rodrigo
    ```

*(Nota: **Fernando** como arquitecto gestionará la rama principal `main` y la integración global en `src/main.rs`).*

### 👣 Paso 3: Implementar tu Módulo de Código
Abre el proyecto en tu editor de código preferido (VS Code, Fleet, etc.) y modifica **únicamente** el archivo que te corresponde en la carpeta `src/`. Esto previene al 100% que sobrescribas el código de tus compañeros.

### 👣 Paso 4: Validar que el Código Compila
Asegúrate de que tus adiciones compilen correctamente ejecutando en la terminal:
```bash
cargo check
```

### 👣 Paso 5: Guardar y Subir tus Cambios a GitHub
Cuando tu implementación esté lista y compilando sin errores, agrégala a Git, haz un commit con un mensaje descriptivo y súbela a tu rama remota:
```bash
git add .
git commit -m "feat: implementar modulo asignado"
git push origin <nombre-de-tu-rama>
```
*(Reemplaza `<nombre-de-tu-rama>` por el nombre exacto de la rama que estás usando, por ejemplo `feature/model-luis`).*

### 👣 Paso 6: Crear el Pull Request (PR) en GitHub
Entra al enlace de tu repositorio en GitHub. Verás un botón verde que dice **"Compare & pull request"**. Presiónalo, describe brevemente lo que implementaste y envía el Pull Request. Fernando revisará el código, lo aprobará e integrará tu trabajo a la rama `main` del proyecto de forma limpia.

---
*¡Mucho éxito, equipo! Mantengamos este flujo ordenado para asegurar la nota máxima tanto en desarrollo técnico como en buenas prácticas de control de versiones.*
