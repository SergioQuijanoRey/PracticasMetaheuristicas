use ndarray::Array;
use std::collections::HashMap;

// Para fijar la semilla de numeros aleatorios
use rand::Rng;

// Reimportamos modulos para poder usar paths como:
// crate::problem_datatypes::Solution en vez de
// crate::problem_datatypes::solution::Solution
mod solution;
mod point;
pub use solution::Solution;
pub use point::Point;

/// Representa el conjunto de puntos que hay que agrupar
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ConstraintType {
    MustLink,
    CannotLink,
}

// TODO -- sobrescribir el tipo de dato (i32, i32) para que sea lo mismo (1, 2) que (2, 1)
/// Estructura de datos que representa las restricciones del problema
/// Usamos un hashmap por motivos de eficiencia la hora de guardar y acceder a los datos
/// Una restriccion viene dada por los dos indices de los elementos que se restringen
/// y el tipo de restriccion
#[derive(Debug, Clone)]
pub struct Constraints{
    data: HashMap<(i32, i32), ConstraintType>,
}

impl Constraints{
    /// Genera una nueva estructura de restricciones con los datos vacios
    /// Es importante usar las funcionalidades de la estructura para no introducir
    /// datos repetidos
    pub fn new() -> Self{
        return Self{data: HashMap::new()};
    }

    /// Añadimos una restriccion, comprobando si ya estaba anteriormente inicializada
    /// Ademas, las restricciones triviales MustLink (i, i) no se consideran
    pub fn add_constraint(&mut self, first_index: i32, second_index: i32, constraint_type: ConstraintType){

        // No añadimos las restricciones triviales MustLink
        if first_index == second_index{
            return ();
        }

        if self.has_element(first_index, second_index) == false {
            self.data.insert((first_index, second_index), constraint_type);
        }
    }

    // Comprueba si tenemos el elemento dado por los indices
    // A mano se comprueba que (a, b) == (b, a) a la hora de mirar las claves
    pub fn has_element(&self, first_index: i32, second_index: i32) -> bool{
        return self.data.contains_key(&(first_index, second_index)) || self.data.contains_key(&(second_index, first_index));
    }

    pub fn get_constraint(&self, first_index: i32, second_index: i32) -> Option<&ConstraintType>{
        // Hacemos los dos if porque no es lo mismo (1, 2) que (2, 1)
        if self.has_element(first_index, second_index) {
            return self.data.get(&(first_index, second_index));

        }
        if self.has_element(second_index, first_index) {
            return self.data.get(&(second_index, first_index));
        }

        // No hay una restriccion entre los dos elementos pasados como parametros
        return None;
    }

    pub fn get_data(&self) -> HashMap<(i32, i32), ConstraintType>{
        return self.data.clone();
    }
}

/// Representa un generador resumido de vecinos
#[derive(Debug)]
pub struct NeighbourGenerator{

    /// El elemento que queremos mover de cluster
    element_index: i32,

    /// El nuevo cluster al que asignamos el elemento
    new_cluster: i32,
}

impl NeighbourGenerator{
    pub fn new(element_index: i32, new_cluster: i32) -> Self{
        return Self {element_index, new_cluster};
    }

    /// Genera todos los posibles vecinos, aunque estos no sean validos, dados
    /// el numero de elementos del dataset y el numero de clusters en los que
    /// queremos dividir dichos elementos
    pub fn generate_all_neighbours(number_of_elements: i32, number_of_clusters: i32) -> Vec<Self>{
        let mut neighbours = vec![];

        for current_element in 0..number_of_elements{
            for current_cluster in 0..number_of_clusters{
                neighbours.push(NeighbourGenerator{
                    element_index: current_element,
                    new_cluster: current_cluster,
                });
            }
        }

        return neighbours;
    }
}
