use ndarray::Array;
use std::collections::HashMap;

// Para fijar la semilla de numeros aleatorios
use rand::Rng;

// Reimportamos modulos para poder usar paths como:
// crate::problem_datatypes::Solution en vez de
// crate::problem_datatypes::solution::Solution
mod solution;
pub use solution::Solution;

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

/// Representa un punto
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    coordinates: ndarray::Array1<f32>,
}

impl Point {
    pub fn new(coordinates: ndarray::Array1<f32>) -> Self{
        return Self{coordinates};

    }

    /// Genera un punto a partir de sus coordenadas dadas en un vector de flotantes
    pub fn from_vec(coordinates: Vec<f32>) -> Self {
        return Self { coordinates: Array::from(coordinates) };
    }


    /// Dados dos puntos, devuelve su distancia euclidea
    pub fn distance(first: &Self, second: &Self) -> f32{
        return first.distance_to(second);
    }

    /// Genera un punto aleatorio cuyas coordenadas se encuentran siempre en
    /// el intervalo [0, 1]
    pub fn random_point(number_of_coordinates: i32) -> Self{
        let mut rng = rand::thread_rng();

        // Array de ceros
        let coordinates: ndarray::Array1<f32> = ndarray::Array1::zeros(number_of_coordinates as usize);

        // En cada elemento colocamos un valor aleatorio
        let coordinates = coordinates.mapv(|_| rng.gen_range(0.0 .. 1.0));

        return Self{coordinates};

    }

    // Dado otro punto, devuelve su distancia euclidea al punto dado
    fn distance_to(&self, other: &Self) -> f32{
        // Hacemos la diferencia en coordenadas
        // Elevamos al cuadrado
        // Sumamos y devolvemos la raiz cuadrada
        let diff = &self.coordinates - &other.coordinates;
        let diff = diff.mapv(|x| x*x);
        return diff.scalar_sum().sqrt();
    }


    /// Dado un conjunto de puntos, calcula su centroide
    // TODO -- TEST -- muy facil de testear
    pub fn calculate_centroid(points: Vec<&Self>) -> Self{
        // Condicion de seguridad
        if points.len() == 0{
            panic!("No se puede calcular el centroide de un conjunto vacio de puntos")
        }

        // Array de zeros con el mismo shape que el primer punto que le pasemos
        let mut sum_point = ndarray::Array1::zeros(points[0].coordinates.len());

        // Calculamos el centroide
        for point in &points{
            sum_point = sum_point + &point.coordinates;
        }
        sum_point = sum_point / points.len() as f32;

        return Self{coordinates: sum_point};
    }

    /// Dado un conjunto de puntos, calcula la maxima distancia entre dos de ellos
    pub fn max_distance_among_two(points: &Vec<Point>) -> f32{
        let mut max_dist = 0.0;

        for i in 0 .. points.len(){
            for j in i .. points.len(){
                let curr_dist = Self::distance(&points[i], &points[j]);

                if curr_dist > max_dist{
                    max_dist = curr_dist;
                }
            }
        }

        return max_dist;

    }

    /// Devuelve la dimension del punto
    /// Es decir, el numero de coordenadas del punto
    pub fn dimension(&self) -> usize {
        return self.coordinates.len();
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
