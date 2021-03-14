pub use crate::problem_datatypes::Point;

/// Representa el conjunto de puntos que hay que agrupar
#[derive(Debug)]
pub struct DataPoints {
    points: Vec<Point>,
}

impl DataPoints {
    pub fn new(points: Vec<Point>) -> Self {
        return Self { points };
    }

    pub fn len(&self) -> usize{
        return self.points.len();
    }

    /// Devuelve la dimension de los puntos del problema
    /// Es decir, el numero de coordenadas de cada punto
    // TODO -- es necesaria esta funcion? Porque podria usar directamente el punto primero
    // de la estructura
    pub fn point_dimension(&self) -> Option<usize>{
        // No tenemos puntos para decir cual es su dimension
        if self.points.len() == 0{
            return None;
        }

        return Some(self.points[0].dimension());
    }

    /// Devuelve una referencia a los puntos que componen el conjunto
    pub fn get_points(&self) -> &Vec<Point>{
        return &self.points;
    }
}

