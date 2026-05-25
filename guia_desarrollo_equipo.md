# Guía Técnica de Desarrollo Colaborativo en Rust (Fase 2)

Esta guía contiene la arquitectura técnica detallada, las responsabilidades específicas, las firmas de código obligatorias, **las ramas de Git asignadas** y el paso a paso exacto para cada uno de los integrantes del equipo. Sigan estas instrucciones al pie de la letra para asegurar la nota máxima (5/5) y que todo compile e integre de forma perfecta y fluida.

---

## 🛠️ 1. Tecnologías y Requisitos del Entorno
*   **Lenguaje**: Rust (compilador estable `rustc` v1.56+ o superior).
*   **Gestor de Proyectos**: Cargo (incluido con Rust).
*   **Librería Externa**: `petgraph` v0.6+ (ya configurada en `Cargo.toml`).
*   **Editor Recomendado**: VS Code con la extensión **rust-analyzer** activa (es vital para el autocompletado y detección de errores de Borrow Checker en tiempo real).

---

## 🗺️ 2. Flujo General de Datos del Proyecto

El proyecto está diseñado bajo un modelo de arquitectura desacoplada. Los módulos interactúan entre sí de la siguiente manera:

```text
  [ src/model.rs (Luis) ]
   Definición de structs base (Ciudad y Conexion)
             │
             ├───> Usado como peso en nodos/aristas en ───> [ src/graph_manual.rs (Mario) ]
             │                                              Estructura de Grafo Manual
             │
             └───> Usado como peso en nodos/aristas en ───> [ src/graph_petgraph.rs (Alberto) ]
                                                            Estructura de Grafo con Petgraph
                                                                        │
  ┌─────────────────────────────────────────────────────────────────────┘
  ▼
[ src/algorithms.rs (Rodrigo) ]
  Algoritmos de recorrido BFS genéricos que consumen las APIs de Mario y Alberto
  │
  ▼
[ src/main.rs (Fernando) ]
  Orquestador global: lee datos, crea grafos, corre BFS y muestra comparativas de rendimiento
```

---

## 🌿 3. Cuadro de Asignación de Roles y Ramas Git

Para evitar conflictos y asegurar que el profesor verifique la participación de cada integrante, **nadie debe trabajar en la rama `main`**. Cada programador tiene asignada una rama y un archivo físico exclusivo:

| Integrante | Rol Técnico | Archivo Asignado | Rama Git en GitHub |
| :--- | :--- | :--- | :--- |
| **Luis** | Modelador del Dominio Vial | `src/model.rs` | `feature/model-luis` |
| **Mario** | Desarrollador del Grafo Manual | `src/graph_manual.rs` | `feature/graph-manual-mario` |
| **Alberto** | Desarrollador de Grafo con Petgraph | `src/graph_petgraph.rs` | `feature/graph-petgraph-alberto` |
| **Rodrigo** | Implementador del Recorrido BFS | `src/algorithms.rs` | `feature/algorithms-rodrigo` |
| **Fernando** | Arquitecto e Integrador Global | `src/main.rs` & `Cargo.toml` | `main` (Gestiona fusiones) |

---

## 👤 4. Especificación Detallada por Desarrollador

---

### 👤 LUIS — Rama: `feature/model-luis`
*   **Archivo Asignado**: `src/model.rs`
*   **Objetivo**: Definir la estructura semántica de la red vial en el mundo real. Tus estructuras serán los "pesos" que se asociarán a los nodos y aristas en los grafos de Mario y Alberto.

#### 👣 Comandos Git para tu flujo de trabajo:
```bash
# 1. Clonar el repositorio y entrar a la carpeta
git clone https://github.com/Fernando-rtx/UltimaEvalucionESD2026.git
cd UltimaEvalucionESD2026

# 2. Cambiarte a tu rama asignada
git checkout feature/model-luis

# 3. Desarrollar tu código en src/model.rs y comprobar que no tiene errores de sintaxis
cargo check

# 4. Guardar y subir tu módulo a GitHub
git add src/model.rs
git commit -m "feat: implementar modelo de dominio Ciudad y Conexion"
git push origin feature/model-luis
```

#### 📋 ¿Qué debe tener tu archivo `src/model.rs`?
Debes definir dos structs principales con constructores y el rasgo `#[derive(Debug, Clone)]` para que todos los compañeros puedan clonar y debugear tus datos fácilmente:

1.  **`struct Ciudad`** (Representa el peso de un **Nodo**):
    ```rust
    #[derive(Debug, Clone)]
    pub struct Ciudad {
        pub id: usize,                  // Identificador numérico único
        pub nombre: String,             // Nombre de la ciudad (ej: "San Salvador")
        pub poblacion: u32,             // Población de la ciudad
        pub coordenadas: (f64, f64),    // Coordenadas (latitud, longitud)
    }

    impl Ciudad {
        pub fn new(id: usize, nombre: &str, poblacion: u32, lat: f64, lon: f64) -> Self {
            Self {
                id,
                nombre: nombre.to_string(),
                poblacion,
                coordenadas: (lat, lon),
            }
        }
    }
    ```

2.  **`struct Conexion`** (Representa el peso de una **Arista** con peso):
    ```rust
    #[derive(Debug, Clone)]
    pub struct Conexion {
        pub distancia: f64,       // Distancia en kilómetros
        pub tiempo_minutos: u32,   // Tiempo estimado de viaje en minutos
        pub costo_peaje: f64,     // Costo de peaje en dólares
    }

    impl Conexion {
        pub fn new(distancia: f64, tiempo_minutos: u32, costo_peaje: f64) -> Self {
            Self {
                distancia,
                tiempo_minutos,
                costo_peaje,
            }
        }
    }
    ```

---

### 👤 MARIO — Rama: `feature/graph-manual-mario`
*   **Archivo Asignado**: `src/graph_manual.rs`
*   **Objetivo**: Diseñar un Grafo Dirigido y Ponderado desde cero, resolviendo las restricciones del **Borrow Checker** sin usar punteros inteligentes (`Rc`, `RefCell`). Implementarás la técnica idiomática de **Arena Allocation (Grafo Basado en Índices sobre Vectores contiguos)**.

#### 👣 Comandos Git para tu flujo de trabajo:
```bash
# 1. Clonar el repositorio y entrar a la carpeta
git clone https://github.com/Fernando-rtx/UltimaEvalucionESD2026.git
cd UltimaEvalucionESD2026

# 2. Cambiarte a tu rama asignada
git checkout feature/graph-manual-mario

# 3. Desarrollar tu código en src/graph_manual.rs y comprobar que compila
cargo check

# 4. Guardar y subir tu módulo a GitHub
git add src/graph_manual.rs
git commit -m "feat: implementar estructura de Grafo Manual por indices de vectores"
git push origin feature/graph-manual-mario
```

#### 📋 ¿Qué debe tener tu archivo `src/graph_manual.rs`?
Debes estructurar el grafo de modo que la relación de adyacencia se rastree mediante una lista enlazada interna donde las "direcciones de memoria" son índices enteros del vector principal.

1.  **Las Estructuras del Grafo**:
    ```rust
    use crate::model::{Ciudad, Conexion};

    #[derive(Debug, Clone)]
    pub struct NodoManual {
        pub peso_nodo: Ciudad,          // La struct Ciudad de Luis
        pub primer_arista: Option<usize>, // Índice de su primera arista en el vector global 'aristas'
    }

    #[derive(Debug, Clone)]
    pub struct AristaManual {
        pub destino: usize,            // Índice del nodo destino en el vector 'nodos'
        pub peso_arista: Conexion,     // La struct Conexion de Luis
        pub siguiente_arista: Option<usize>, // Índice de la siguiente arista que sale del mismo origen
    }

    #[derive(Debug, Clone)]
    pub struct GrafoManual {
        pub nodos: Vec<NodoManual>,
        pub aristas: Vec<AristaManual>,
    }
    ```

2.  **Los Métodos de Implementación Obligatorios**:
    ```rust
    impl GrafoManual {
        pub fn new() -> Self {
            Self {
                nodos: Vec::new(),
                aristas: Vec::new(),
            }
        }

        // Agrega un nodo al vector global y retorna su índice (usize)
        pub fn agregar_nodo(&mut self, ciudad: Ciudad) -> usize {
            let idx = self.nodos.len();
            self.nodos.push(NodoManual {
                peso_nodo: ciudad,
                primer_arista: None,
            });
            idx
        }

        // Agrega una arista dirigida ponderada usando listas enlazadas por índices
        pub fn agregar_arista(&mut self, origen: usize, destino: usize, conexion: Conexion) {
            let nueva_arista_idx = self.aristas.len();
            
            // Creamos la arista. Apunta a la que era la "primera arista" del origen para insertarla al inicio
            let siguiente = self.nodos[origen].primer_arista;
            self.aristas.push(AristaManual {
                destino,
                peso_arista: conexion,
                siguiente_arista: siguiente,
            });

            // Ahora, la nueva arista es la primera del nodo origen
            self.nodos[origen].primer_arista = Some(nueva_arista_idx);
        }

        // Retorna un vector con los índices de los vecinos adyacentes y sus conexiones asociadas
        pub fn obtener_vecinos(&self, nodo_idx: usize) -> Vec<(usize, &Conexion)> {
            let mut vecinos = Vec::new();
            let mut arista_actual = self.nodos[nodo_idx].primer_arista;

            while let Some(idx) = arista_actual {
                let arista = &self.aristas[idx];
                vecinos.push((arista.destino, &arista.peso_arista));
                arista_actual = arista.siguiente_arista;
            }
            vecinos
        }
    }
    ```

---

### 👤 ALBERTO — Rama: `feature/graph-petgraph-alberto`
*   **Archivo Asignado**: `src/graph_petgraph.rs`
*   **Objetivo**: Implementar el mismo modelado de grafo dirigido y ponderado pero utilizando nativamente la crate estándar de la industria: **`petgraph`**.

#### 👣 Comandos Git para tu flujo de trabajo:
```bash
# 1. Clonar el repositorio y entrar a la carpeta
git clone https://github.com/Fernando-rtx/UltimaEvalucionESD2026.git
cd UltimaEvalucionESD2026

# 2. Cambiarte a tu rama asignada
git checkout feature/graph-petgraph-alberto

# 3. Desarrollar tu código en src/graph_petgraph.rs y comprobar que compila
cargo check

# 4. Guardar y subir tu módulo a GitHub
git add src/graph_petgraph.rs
git commit -m "feat: implementar encapsulador de Grafo usando biblioteca petgraph"
git push origin feature/graph-petgraph-alberto
```

#### 📋 ¿Qué debe tener tu archivo `src/graph_petgraph.rs`?
Importarás las estructuras genéricas de `petgraph` y las encapsularás para proveer una API idéntica a la de Mario, demostrando el contraste directo entre el desarrollo a bajo nivel y con librería.

1.  **La Estructura**:
    ```rust
    use crate::model::{Ciudad, Conexion};
    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    use petgraph::Directed;

    pub struct GrafoPetgraph {
        // Grafo dirigido con pesos en nodos (Ciudad) y aristas (Conexion), optimizado con índices u32
        pub grafo: Graph<Ciudad, Conexion, Directed, u32>,
    }
    ```

2.  **Los Métodos de Implementación Obligatorios**:
    ```rust
    impl GrafoPetgraph {
        pub fn new() -> Self {
            Self {
                grafo: Graph::new(),
            }
        }

        // Agrega un nodo usando la librería y retorna su NodeIndex
        pub fn agregar_nodo(&mut self, ciudad: Ciudad) -> NodeIndex<u32> {
            self.grafo.add_node(ciudad)
        }

        // Agrega una arista dirigida entre dos nodos usando sus NodeIndex
        pub fn agregar_arista(&mut self, origen: NodeIndex<u32>, destino: NodeIndex<u32>, conexion: Conexion) {
            self.grafo.add_edge(origen, destino, conexion);
        }

        // Obtiene la lista de vecinos adyacentes de forma segura usando los iteradores de petgraph
        pub fn obtener_vecinos(&self, nodo: NodeIndex<u32>) -> Vec<(NodeIndex<u32>, &Conexion)> {
            let mut vecinos = Vec::new();
            
            // Iteramos sobre las aristas salientes del nodo especificado
            let mut edges = self.grafo.neighbors_directed(nodo, petgraph::Direction::Outgoing).detach();
            while let Some((edge_idx, neighbor_idx)) = edges.next(&self.grafo) {
                if let Some(conexion) = self.grafo.edge_weight(edge_idx) {
                    vecinos.push((neighbor_idx, conexion));
                }
            }
            vecinos
        }
    }
    ```

---

### 👤 RODRIGO — Rama: `feature/algorithms-rodrigo`
*   **Archivo Asignado**: `src/algorithms.rs`
*   **Objetivo**: Codificar un algoritmo de recorrido clásico de teoría de grafos. Implementarás el algoritmo **BFS (Breadth-First Search - Búsqueda en Anchura)**, el cual es ideal para encontrar caminos más cortos en cantidad de saltos en redes viales.

#### 👣 Comandos Git para tu flujo de trabajo:
```bash
# 1. Clonar el repositorio y entrar a la carpeta
git clone https://github.com/Fernando-rtx/UltimaEvalucionESD2026.git
cd UltimaEvalucionESD2026

# 2. Cambiarte a tu rama asignada
git checkout feature/algorithms-rodrigo

# 3. Desarrollar tu código en src/algorithms.rs y comprobar que compila
cargo check

# 4. Guardar y subir tu módulo a GitHub
git add src/algorithms.rs
git commit -m "feat: implementar algoritmos BFS para Grafo Manual y Petgraph"
git push origin feature/algorithms-rodrigo
```

#### 📋 ¿Qué debe tener tu archivo `src/algorithms.rs`?
Implementarás dos funciones completamente desacopladas de algoritmos de recorrido: una que interactúe con el grafo de Mario (`GrafoManual`) y otra con el de Alberto (`GrafoPetgraph`). Utilizarás una cola `VecDeque` para el procesamiento FIFO de nodos.

1.  **Algoritmo BFS para el Grafo Manual de Mario**:
    ```rust
    use crate::graph_manual::GrafoManual;
    use std::collections::{VecDeque, HashSet};

    // Realiza un recorrido en anchura desde un nodo inicial y retorna los índices visitados en orden
    pub fn bfs_manual(grafo: &GrafoManual, inicio_idx: usize) -> Vec<usize> {
        let mut visitados = HashSet::new();
        let mut cola = VecDeque::new();
        let mut resultado = Vec::new();

        cola.push_back(inicio_idx);
        visitados.insert(inicio_idx);

        while let Some(nodo_actual) = cola.pop_front() {
            resultado.push(nodo_actual);

            // Obtenemos los vecinos directos llamando a la función de Mario
            for (vecino_idx, _) in grafo.obtener_vecinos(nodo_actual) {
                if !visitados.contains(&vecino_idx) {
                    visitados.insert(vecino_idx);
                    cola.push_back(vecino_idx);
                }
            }
        }
        resultado
    }
    ```

2.  **Algoritmo BFS para el Grafo de petgraph de Alberto**:
    ```rust
    use crate::graph_petgraph::GrafoPetgraph;
    use petgraph::graph::NodeIndex;
    use std::collections::{VecDeque, HashSet};

    // Realiza el mismo recorrido BFS pero sobre la estructura de la librería petgraph
    pub fn bfs_petgraph(grafo: &GrafoPetgraph, inicio: NodeIndex<u32>) -> Vec<NodeIndex<u32>> {
        let mut visitados = HashSet::new();
        let mut cola = VecDeque::new();
        let mut resultado = Vec::new();

        cola.push_back(inicio);
        visitados.insert(inicio);

        while let Some(nodo_actual) = cola.pop_front() {
            resultado.push(nodo_actual);

            // Obtenemos los vecinos directos llamando a la función de Alberto
            for (vecino, _) in grafo.obtener_vecinos(nodo_actual) {
                if !visitados.contains(&vecino) {
                    visitados.insert(vecino);
                    cola.push_back(vecino);
                }
            }
        }
        resultado
    }
    ```

---

## 🚀 5. Integración y Pruebas (Orquestado por Fernando en la rama `main`)

Una vez que Luis, Mario, Alberto y Rodrigo hayan subido sus respectivos módulos a Git y se hayan integrado en la rama `main` tras una Pull Request, Fernando implementará en `src/main.rs` un flujo de orquestación integrado para inicializar los datos, construir ambos grafos y correr las búsquedas comparando el rendimiento y resultados.

¡Sigamos este mapa al pie de la letra para asegurar una entrega perfecta de la Fase 2!
