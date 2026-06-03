// Módulo graph_petgraph.rs - Asignado a Alberto
// Aquí se implementará el grafo utilizando la librería petgraph.
use crate::model::{Ciudad, Conexion};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use petgraph::Directed;

pub struct GrafoPetgraph {
    pub grafo: Graph<Ciudad, Conexion, Directed, u32>,
}

impl GrafoPetgraph {
    pub fn new() -> Self {
        Self {
            grafo: Graph::new(),
        }
    }

    pub fn agregar_nodo(&mut self, ciudad: Ciudad) -> NodeIndex<u32> {
        self.grafo.add_node(ciudad)
    }

    pub fn agregar_arista(
        &mut self,
        origen: NodeIndex<u32>,
        destino: NodeIndex<u32>,
        conexion: Conexion,
    ) {
        self.grafo.add_edge(origen, destino, conexion);
    }

    pub fn obtener_vecinos(&self, nodo: NodeIndex<u32>) -> Vec<(NodeIndex<u32>, &Conexion)> {
        let mut vecinos = Vec::new();

        let mut edges = self
            .grafo
            .neighbors_directed(nodo, Direction::Outgoing)
            .detach();

        while let Some((edge_idx, neighbor_idx)) = edges.next(&self.grafo) {
            if let Some(conexion) = self.grafo.edge_weight(edge_idx) {
                vecinos.push((neighbor_idx, conexion));
            }
        }

        vecinos
    }
}
