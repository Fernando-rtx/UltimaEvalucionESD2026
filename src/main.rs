// Declaración de los módulos del proyecto de grafos
pub mod model;
pub mod graph_manual;
pub mod graph_petgraph;
pub mod algorithms;

use model::{Ciudad, Conexion};
use graph_manual::GrafoManual;
use graph_petgraph::GrafoPetgraph;
//use algorithms::{bfs_manual, bfs_petgraph};
use algorithms::{bfs_manual, bfs_petgraph, dfs_manual, dfs_petgraph};
use std::time::Instant;

fn main() {
    println!("============================================================");
    println!("SISTEMA DE ANÁLISIS DE REDES VIALES (RUST) ");
    println!("============================================================");

    // 1. Instanciamos las ciudades (Nodos) - Trabajo de Luis
    let san_salvador = Ciudad::new(0, "San Salvador", 1_100_000, 13.6929, -89.2182);
    let santa_ana = Ciudad::new(1, "Santa Ana", 280_000, 13.9778, -89.5597);
    let san_miguel = Ciudad::new(2, "San Miguel", 260_000, 13.4833, -88.1833);
    let sonsonate = Ciudad::new(3, "Sonsonate", 110_000, 13.7189, -89.7242);
    let la_libertad = Ciudad::new(4, "La Libertad", 50_000, 13.4883, -89.3147);
    let usulutan = Ciudad::new(5, "Usulután", 80_000, 13.3444, -88.4392);

    println!("\n[1] Nodos Creados: Integración del modelo de Luis (model.rs) ✅");

    // 2. Construcción del Grafo Manual (Arena Allocation) - Trabajo de Mario
    println!("\n[2] Construyendo Grafo Manual (Arena Allocation) - Mario...");
    let mut grafo_manual = GrafoManual::new();
    
    let ss_idx = grafo_manual.agregar_nodo(san_salvador.clone());
    let sa_idx = grafo_manual.agregar_nodo(santa_ana.clone());
    let sm_idx = grafo_manual.agregar_nodo(san_miguel.clone());
    let so_idx = grafo_manual.agregar_nodo(sonsonate.clone());
    let la_idx = grafo_manual.agregar_nodo(la_libertad.clone());
    let us_idx = grafo_manual.agregar_nodo(usulutan.clone());

    // Añadimos aristas manuales
    grafo_manual.agregar_arista(ss_idx, sa_idx, Conexion::new(65.0, 60, 0.0));
    grafo_manual.agregar_arista(ss_idx, sm_idx, Conexion::new(138.0, 150, 0.0));
    grafo_manual.agregar_arista(ss_idx, la_idx, Conexion::new(34.0, 45, 0.0));
    grafo_manual.agregar_arista(sa_idx, so_idx, Conexion::new(40.0, 40, 0.0));
    grafo_manual.agregar_arista(sm_idx, us_idx, Conexion::new(52.0, 60, 0.0));
    grafo_manual.agregar_arista(la_idx, so_idx, Conexion::new(70.0, 90, 0.0));
    grafo_manual.agregar_arista(la_idx, us_idx, Conexion::new(90.0, 120, 0.0));
    println!("     Grafo Manual construido con {} nodos y listas de adyacencia ligadas.", grafo_manual.nodos.len());

    // 3. Construcción del Grafo Petgraph (Librería Industrial) - Trabajo de Alberto
    println!("\n[3] Construyendo Grafo Petgraph (Librería Industrial) - Alberto...");
    let mut grafo_petgraph = GrafoPetgraph::new();
    
    let ss_node = grafo_petgraph.agregar_nodo(san_salvador.clone());
    let sa_node = grafo_petgraph.agregar_nodo(santa_ana.clone());
    let sm_node = grafo_petgraph.agregar_nodo(san_miguel.clone());
    let so_node = grafo_petgraph.agregar_nodo(sonsonate.clone());
    let la_node = grafo_petgraph.agregar_nodo(la_libertad.clone());
    let us_node = grafo_petgraph.agregar_nodo(usulutan.clone());

    grafo_petgraph.agregar_arista(ss_node, sa_node, Conexion::new(65.0, 60, 0.0));
    grafo_petgraph.agregar_arista(ss_node, sm_node, Conexion::new(138.0, 150, 0.0));
    grafo_petgraph.agregar_arista(ss_node, la_node, Conexion::new(34.0, 45, 0.0));
    grafo_petgraph.agregar_arista(sa_node, so_node, Conexion::new(40.0, 40, 0.0));
    grafo_petgraph.agregar_arista(sm_node, us_node, Conexion::new(52.0, 60, 0.0));
    grafo_petgraph.agregar_arista(la_node, so_node, Conexion::new(70.0, 90, 0.0));
    grafo_petgraph.agregar_arista(la_node, us_node, Conexion::new(90.0, 120, 0.0));
    println!("     Grafo Petgraph construido encapsulando la librería industrial.");

   // 4. Ejecutando Algoritmos de Búsqueda en Anchura (BFS) - Trabajo de Rodrigo
    println!("\n[4] Ejecutando Algoritmos de Recorrido BFS - Rodrigo...");

    let start_manual = Instant::now();
    let ruta_manual = bfs_manual(&grafo_manual, ss_idx);
    let duration_manual = start_manual.elapsed();

    print!("     Ruta (BFS Manual):   ");
    for (i, idx) in ruta_manual.iter().enumerate() {
        print!("{}", grafo_manual.nodos[*idx].peso_nodo.nombre);
        if i < ruta_manual.len() - 1 { print!(" -> "); }
    }
    println!("\n       (Tiempo de ejecución: {:?})", duration_manual);

    let start_petgraph = Instant::now();
    let ruta_petgraph = bfs_petgraph(&grafo_petgraph, ss_node);
    let duration_petgraph = start_petgraph.elapsed();

    print!("     Ruta (BFS Petgraph): ");
    for (i, idx) in ruta_petgraph.iter().enumerate() {
        if let Some(ciudad) = grafo_petgraph.grafo.node_weight(*idx) {
            print!("{}", ciudad.nombre);
            if i < ruta_petgraph.len() - 1 { print!(" -> "); }
        }
    }
    println!("\n       (Tiempo de ejecución: {:?})", duration_petgraph);

    // 5. Ejecutando Algoritmos de Búsqueda en Profundidad (DFS) - Trabajo de Rodrigo
    println!("\n[5] Ejecutando Algoritmos de Recorrido DFS - Rodrigo...");

    let start_dfs_manual = Instant::now();
    let ruta_dfs_manual = dfs_manual(&grafo_manual, ss_idx);
    let duration_dfs_manual = start_dfs_manual.elapsed();

    print!("     Ruta (DFS Manual):   ");
    for (i, idx) in ruta_dfs_manual.iter().enumerate() {
        print!("{}", grafo_manual.nodos[*idx].peso_nodo.nombre);
        if i < ruta_dfs_manual.len() - 1 { print!(" -> "); }
    }
    println!("\n       (Tiempo de ejecución: {:?})", duration_dfs_manual);

    let start_dfs_petgraph = Instant::now();
    let ruta_dfs_petgraph = dfs_petgraph(&grafo_petgraph, ss_node);
    let duration_dfs_petgraph = start_dfs_petgraph.elapsed();

    print!("     Ruta (DFS Petgraph): ");
    for (i, idx) in ruta_dfs_petgraph.iter().enumerate() {
        if let Some(ciudad) = grafo_petgraph.grafo.node_weight(*idx) {
            print!("{}", ciudad.nombre);
            if i < ruta_dfs_petgraph.len() - 1 { print!(" -> "); }
        }
    }
    println!("\n       (Tiempo de ejecución: {:?})", duration_dfs_petgraph);

    println!("\n============================================================");
    println!(" ¡ORQUESTACIÓN FINAL EXITOSA - PROYECTO COMPLETADO! ");
    println!("============================================================");
}
