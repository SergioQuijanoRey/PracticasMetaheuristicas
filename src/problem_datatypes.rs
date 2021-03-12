use ndarray::Array;
use std::collections::HashMap;

// Para fijar la semilla de numeros aleatorios
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

// Para hacer shuffle de un vector
use rand::thread_rng;
use rand::seq::SliceRandom;

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
}

/// Representa un punto
#[derive(Debug, Clone)]
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

/// Estructura que representa una solucion del problema
///
/// La solucion viene representada como un vector de indices
/// En dicho vector, la posicion i-esima indica el cluster al que pertenece el i-esimo
/// punto del conjunto de datos
#[derive(Debug, Clone)]
pub struct Solution {
    cluster_indexes: Vec<i32>,
    data_points: DataPoints,
    constraints: Constraints,
    number_of_clusters: i32,

    /// Representa el peso de infeasibility en el calculo de fitness
    /// Solo se calcula una vez al invocar a Solution::new
    lambda: f32,

    /// Necesitamos poder establecer el valor de la semilla para hacer comparaciones
    seed: i32,
}

impl Solution {
    /// Util cuando no conocemos el valor de lambda, pues se calcula en esta llamada
    /// En otro caso, se puede construir el struct de forma directa
    pub fn new(
        cluster_indexes: Vec<i32>,
        data_points: DataPoints,
        constraints: Constraints,
        number_of_clusters: i32,
        seed: i32,
    ) -> Self {

        // Calculamos el valor de lambda
        let lambda = Point::max_distance_among_two(&data_points.points) / constraints.data.len() as f32;

        return Self {
            cluster_indexes,
            data_points,
            constraints,
            number_of_clusters,
            lambda,
            seed,
        };
    }

    pub fn get_cluster_indexes(&self) -> Vec<i32>{
        return self.cluster_indexes.clone();
    }

    pub fn get_lambda(&self) -> f32{
        return self.lambda;
    }

    /// Comprueba si la solucion es valida o no
    fn is_valid(&self) -> bool {

        // Condicion de seguridad que nunca deberia ocurrir
        // Por eso pongo el panic!, porque es un problema de probramacion
        if self.cluster_indexes.len() != self.data_points.points.len(){
            panic!("No puede ocurrir que la longitud de los indices sea distinta al numero de puntos");
        }

        // Comprobamos que no haya clusters vacios
        for cluster in 0..self.number_of_clusters{
            match self.cluster_indexes.iter().find(|&&x| x == cluster){
                // Se ha encontrado, no hacemos nada
                Some(_) =>(),

                // No hemos encontrado ningun valor de indice que apunte a este cluster
                None => return false,
            }
        }

        // No hemos encontrado cluster vacio
        return true;
    }

    /// Calcula el valor de fitness de la solucion
    pub fn fitness(&self) -> f32 {
        return self.global_cluster_mean_distance() + self.lambda * self.infeasibility() as f32;
    }

    /// Devuelve el primer vecino de la solucion valido que mejora la solucion
    /// actual (el primero mejor)
    pub fn get_neighbour(&self) -> Option<Self> {
        // Generador de numeros aleatorios
        // TODO -- da problemas el fijar la semilla aleatoria
        let mut rng = Self::fix_random_seed(self.seed);
        let mut rng = rand::thread_rng();

        // Tomo los generadores de vecinos
        let mut neighbours_generator = NeighbourGenerator::generate_all_neighbours(self.data_points.len() as i32, self.number_of_clusters);

        // Mezclo los generadores de vecinos
        neighbours_generator.shuffle(&mut rng);

        for current_generator in neighbours_generator{
            let current_solution = self.generate_solution_from(current_generator);

            if current_solution.is_valid() && current_solution.fitness() < self.fitness(){
                return Some(current_solution);
            }
        }

        // No hemos encontrado un vecino mejor
        return None;
    }

    /// A partir de un NeighbourGenerator, genera la solucion que representa el
    /// generador aplicado a la solucion &self
    fn generate_solution_from(&self, generator: NeighbourGenerator) -> Self{
        let mut new_solution = self.clone();
        new_solution.cluster_indexes[generator.element_index as usize] = generator.new_cluster;
        return new_solution;
    }

    /// Genera una solucion inicial aleatoria, como punto de partida de las busquedas
    // TODO -- no puede dejar clusters vacios
    pub fn generate_random_solution(
        data_points: DataPoints,
        constraints: Constraints,
        number_of_clusters: i32,
        seed: i32
    ) -> Self {
        // Generador de numeros aleatorios
        // TODO -- da problemas fijar la semilla aleatoria
        let mut rng = Self::fix_random_seed(seed);
        let mut rng = rand::thread_rng();

        return Self::new(
            (0..data_points.points.len()).map(|_| rng.gen_range(0..number_of_clusters)).collect(),
            data_points,
            constraints,
            number_of_clusters,
            seed,
        );

    }

    /// Dado un cluster (representado por el entero que los identifica), calcula
    /// la distancia intracluster en la solucion actual
    // TODO -- comprobar que no estemos dividiendo por cero, ya sea con un result
    // o con un panic!
    pub fn intra_cluster_distance(&self, cluster: i32) -> f32{
        // Calculamos el vector de puntos que estan en el cluster
        let cluster_points = self.get_points_in_cluster(cluster);

        // Calculamos el centroide de dicho conjunto de puntos
        let centroid = Point::calculate_centroid(cluster_points.clone());

        // Calculamos la distancia intracluster
        let mut cum_sum = 0.0;
        for point in &cluster_points{
            cum_sum += Point::distance(point, &centroid);
        }
        return cum_sum / cluster_points.len() as f32;

    }

    /// Dado un cluster indicado por el indice que lo representa, devuelve los puntos
    /// que componen dicho cluster
    // TODO -- TEST -- esta funcion es muy facil de testear
    pub fn get_points_in_cluster(&self, cluster: i32) -> Vec<&Point>{
        let mut cluster_points = vec![];

        for (index, curr_cluster) in self.cluster_indexes.iter().enumerate(){
            if *curr_cluster == cluster{
                cluster_points.push(&self.data_points.points[index]);
            }
        }

        return cluster_points;
    }

    /// Calcula la media de distancias intracluster sobre todos los clusters
    /// Esto representa una de las componentes de la funcion fitness
    pub fn global_cluster_mean_distance(&self) -> f32{
        let mut cum_sum = 0.0;
        for i in 0 .. self.number_of_clusters{
            cum_sum += self.intra_cluster_distance(i);
        }

        return cum_sum / self.number_of_clusters as f32;
    }

    /// Calcula el numero de restricciones que se violan en la solucion actual
    pub fn infeasibility(&self) -> i32{
        let mut infea = 0;
        for ((first_index, second_index), value) in &self.constraints.get_data(){

            // Tomamos los dos indices de cluster para compararlos
            let first_cluster = self.cluster_indexes[*first_index as usize];
            let second_cluster  = self.cluster_indexes[*second_index as usize];

            match value{
                ConstraintType::MustLink => {
                    // Sumamos cuando no estan en el mismo cluster
                    if first_cluster != second_cluster{
                        infea += 1;
                    }
                }

                ConstraintType::CannotLink => {
                    // Sumamos cuando estan en el mismo cluster
                    if first_cluster == second_cluster{
                        infea += 1;
                    }
                }
            }
        }

        return infea;
    }

    // TODO -- no funciona como deberia
    // Cada vez que lo llamo, genera una nueva semilla. Deberia guardar el generador
    // de numeros aleatorios en un campo propio
    fn fix_random_seed(seed: i32) -> rand::rngs::StdRng{
        return StdRng::seed_from_u64(seed as u64);
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
