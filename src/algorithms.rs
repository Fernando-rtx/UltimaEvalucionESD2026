// src/algorithms.rs
// Rodrigo — Implementación de algoritmos BFS y DFS para Grafo Manual y Petgraph

use crate::graph_manual::GrafoManual;
use crate::graph_petgraph::GrafoPetgraph;
use petgraph::graph::NodeIndex;
use std::collections::{HashSet, VecDeque};

// ─────────────────────────────────────────────
//  BFS — Grafo Manual (Mario)
// ─────────────────────────────────────────────
pub fn bfs_manual(grafo: &GrafoManual, inicio_idx: usize) -> Vec<usize> {
    let mut visitados: HashSet<usize> = HashSet::new();
    let mut cola: VecDeque<usize> = VecDeque::new();
    let mut resultado: Vec<usize> = Vec::new();

    cola.push_back(inicio_idx);
    visitados.insert(inicio_idx);

    while let Some(nodo_actual) = cola.pop_front() {
        resultado.push(nodo_actual);

        for (vecino_idx, _conexion) in grafo.obtener_vecinos(nodo_actual) {
            if !visitados.contains(&vecino_idx) {
                visitados.insert(vecino_idx);
                cola.push_back(vecino_idx);
            }
        }
    }

    resultado
}

// ─────────────────────────────────────────────
//  BFS — Grafo Petgraph (Alberto)
// ─────────────────────────────────────────────
pub fn bfs_petgraph(grafo: &GrafoPetgraph, inicio: NodeIndex<u32>) -> Vec<NodeIndex<u32>> {
    let mut visitados: HashSet<NodeIndex<u32>> = HashSet::new();
    let mut cola: VecDeque<NodeIndex<u32>> = VecDeque::new();
    let mut resultado: Vec<NodeIndex<u32>> = Vec::new();

    cola.push_back(inicio);
    visitados.insert(inicio);

    while let Some(nodo_actual) = cola.pop_front() {
        resultado.push(nodo_actual);

        for (vecino, _conexion) in grafo.obtener_vecinos(nodo_actual) {
            if !visitados.contains(&vecino) {
                visitados.insert(vecino);
                cola.push_back(vecino);
            }
        }
    }

    resultado
}

// ─────────────────────────────────────────────
//  DFS — Grafo Manual (Mario)
// ─────────────────────────────────────────────
// Usa un Stack (Vec) en lugar de una cola.
// Saca siempre del tope → profundiza antes de explorar vecinos.
pub fn dfs_manual(grafo: &GrafoManual, inicio_idx: usize) -> Vec<usize> {
    let mut visitados: HashSet<usize> = HashSet::new();
    let mut pila: Vec<usize> = Vec::new();
    let mut resultado: Vec<usize> = Vec::new();

    pila.push(inicio_idx);

    while let Some(nodo_actual) = pila.pop() {
        // Si ya fue visitado (puede entrar dos veces al stack), lo saltamos
        if visitados.contains(&nodo_actual) {
            continue;
        }

        visitados.insert(nodo_actual);
        resultado.push(nodo_actual);

        // Empujamos los vecinos al stack (en orden inverso para mantener
        // el mismo orden de visita que tendría la versión recursiva)
        let mut vecinos: Vec<usize> = grafo
            .obtener_vecinos(nodo_actual)
            .iter()
            .map(|(idx, _)| *idx)
            .collect();
        vecinos.reverse();

        for vecino_idx in vecinos {
            if !visitados.contains(&vecino_idx) {
                pila.push(vecino_idx);
            }
        }
    }

    resultado
}

// ─────────────────────────────────────────────
//  DFS — Grafo Petgraph (Alberto)
// ─────────────────────────────────────────────
pub fn dfs_petgraph(grafo: &GrafoPetgraph, inicio: NodeIndex<u32>) -> Vec<NodeIndex<u32>> {
    let mut visitados: HashSet<NodeIndex<u32>> = HashSet::new();
    let mut pila: Vec<NodeIndex<u32>> = Vec::new();
    let mut resultado: Vec<NodeIndex<u32>> = Vec::new();

    pila.push(inicio);

    while let Some(nodo_actual) = pila.pop() {
        if visitados.contains(&nodo_actual) {
            continue;
        }

        visitados.insert(nodo_actual);
        resultado.push(nodo_actual);

        let mut vecinos: Vec<NodeIndex<u32>> = grafo
            .obtener_vecinos(nodo_actual)
            .iter()
            .map(|(idx, _)| *idx)
            .collect();
        vecinos.reverse();

        for vecino in vecinos {
            if !visitados.contains(&vecino) {
                pila.push(vecino);
            }
        }
    }

    resultado
}
