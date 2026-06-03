#[derive(Debug, Clone)]
pub struct Ciudad {
    pub id: usize,
    pub nombre: String,
    pub poblacion: u32,
    pub coordenadas: (f64, f64),
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

#[derive(Debug, Clone)]
pub struct Conexion {
    pub distancia: f64,
    pub tiempo_minutos: u32,
    pub costo_peaje: f64,
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