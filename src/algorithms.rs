// src/algorithms.rs
// Rodrigo — Implementación de algoritmos BFS para Grafo Manual y Petgraph

use crate::graph_manual::GrafoManual;
use crate::graph_petgraph::GrafoPetgraph;
use petgraph::graph::NodeIndex;
use std::collections::{HashSet, VecDeque};

// BFS sobre el grafo manual de Mario.
// Recibe el grafo y el índice del nodo de inicio.
// Retorna los índices de los nodos visitados en orden de anchura.
pub fn bfs_manual(grafo: &GrafoManual, inicio_idx: usize) -> Vec<usize> {
    let mut visitados: HashSet<usize> = HashSet::new();
    let mut cola: VecDeque<usize> = VecDeque::new();
    let mut resultado: Vec<usize> = Vec::new();

    // Metemos el nodo inicial a la cola y lo marcamos como visitado
    cola.push_back(inicio_idx);
    visitados.insert(inicio_idx);

    // Mientras haya nodos en la cola, seguimos explorando
    while let Some(nodo_actual) = cola.pop_front() {
        resultado.push(nodo_actual);

        // Preguntamos al grafo de Mario cuáles son los vecinos del nodo actual
        for (vecino_idx, _conexion) in grafo.obtener_vecinos(nodo_actual) {
            if !visitados.contains(&vecino_idx) {
                visitados.insert(vecino_idx);
                cola.push_back(vecino_idx);
            }
        }
    }

    resultado
}

// BFS sobre el grafo de petgraph de Alberto.
// Funciona igual que el anterior pero usando los tipos de la librería petgraph.
pub fn bfs_petgraph(grafo: &GrafoPetgraph, inicio: NodeIndex<u32>) -> Vec<NodeIndex<u32>> {
    let mut visitados: HashSet<NodeIndex<u32>> = HashSet::new();
    let mut cola: VecDeque<NodeIndex<u32>> = VecDeque::new();
    let mut resultado: Vec<NodeIndex<u32>> = Vec::new();

    cola.push_back(inicio);
    visitados.insert(inicio);

    while let Some(nodo_actual) = cola.pop_front() {
        resultado.push(nodo_actual);

        // Preguntamos al grafo de Alberto cuáles son los vecinos
        for (vecino, _conexion) in grafo.obtener_vecinos(nodo_actual) {
            if !visitados.contains(&vecino) {
                visitados.insert(vecino);
                cola.push_back(vecino);
            }
        }
    }

    resultado
}