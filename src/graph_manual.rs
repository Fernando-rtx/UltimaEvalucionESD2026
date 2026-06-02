use crate::model::{Ciudad, Conexion};

#[derive(Debug, Clone)]
pub struct NodoManual {
    pub peso_nodo: Ciudad,
    pub primer_arista: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct AristaManual {
    pub destino: usize,
    pub peso_arista: Conexion,
    pub siguiente_arista: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct GrafoManual {
    pub nodos: Vec<NodoManual>,
    pub aristas: Vec<AristaManual>,
}

impl GrafoManual {
    pub fn new() -> Self {
        Self {
            nodos: Vec::new(),
            aristas: Vec::new(),
        }
    }

    pub fn agregar_nodo(&mut self, ciudad: Ciudad) -> usize {
        let idx = self.nodos.len();
        self.nodos.push(NodoManual {
            peso_nodo: ciudad,
            primer_arista: None,
        });
        idx
    }

    pub fn agregar_arista(&mut self, origen: usize, destino: usize, conexion: Conexion) {
        let nueva_arista_idx = self.aristas.len();
        
        let siguiente = self.nodos[origen].primer_arista;
        self.aristas.push(AristaManual {
            destino,
            peso_arista: conexion,
            siguiente_arista: siguiente,
        });

        self.nodos[origen].primer_arista = Some(nueva_arista_idx);
    }

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
