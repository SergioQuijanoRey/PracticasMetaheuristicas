use rand::Rng;

/// Representa el conjunto de puntos que hay que agrupar
#[derive(Debug, Clone)]
pub struct DataPoints {
    points: Vec<Point>,
}

impl DataPoints {
    pub fn new(points: Vec<Point>) -> Self {
        return Self { points };
    }
}

/// Representa un punto
// TODO -- no deberia tener campos publicos
#[derive(Debug, Clone)]
pub struct Point {
    coordinates: Vec<f32>,
}

impl Point {
    pub fn new(coordinates: Vec<f32>) -> Self {
        return Self { coordinates };
    }
}

#[derive(Debug, Clone)]
pub enum ConstraintType {
    MustLink,
    CannotLink,
}

/// Estructura de datos que representa una restriccion
/// Una restriccion viene dada por los dos indices de los elementos que se
/// restringen y el tipo de restriccion
// TODO -- pasar esta estructura de datos a un hash para tener acceso directo
#[derive(Debug, Clone)]
pub struct Constraint {
    first_index: i32,
    second_index: i32,
    constraint_type: ConstraintType,
}

impl Constraint {
    pub fn new(first_index: i32, second_index: i32, constraint_type: ConstraintType) -> Self {
        return Self {
            first_index,
            second_index,
            constraint_type,
        };
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
    constraints: Vec<Constraint>,
    number_of_clusters: i32,
}

/// TODO -- falta por implementar
impl Solution {
    pub fn new(
        cluster_indexes: Vec<i32>,
        data_points: DataPoints,
        constraints: Vec<Constraint>,
        number_of_clusters: i32
    ) -> Self {
        return Self {
            cluster_indexes,
            data_points,
            constraints,
            number_of_clusters
        };
    }

    pub fn get_cluster_indexes(&self) -> Vec<i32>{
        return self.cluster_indexes.clone();
    }

    /// Comprueba si la solucion es valida o no
    // TODO -- siempre devuelve true, lo cual no es valido
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
    // TODO -- no es la funcion que deberia ser
    pub fn fitness(&self) -> f32 {
        let mut fitness = 0.0;
        for value in &self.cluster_indexes{
            fitness = fitness + (*value as f32)
        }

        return fitness;
    }

    /// Devuelve un vecino de la solucion
    // TODO -- genera muchas soluciones no validas
    pub fn get_neighbour(&self) -> Self {
        // Generador de numeros aleatorios
        let mut rng = rand::thread_rng();

        // Indice que queremos cambiar
        let index_to_change = rng.gen_range(0..self.cluster_indexes.len());

        // Nuevo valor del indice
        let new_index = rng.gen_range(0..self.number_of_clusters);

        // Tomamos el nuevo vector de indices con el cambio
        let mut new_cluster_indexes = self.cluster_indexes.clone();
        new_cluster_indexes[index_to_change] = new_index;


        let neighbour = Self {
            cluster_indexes: new_cluster_indexes,
            data_points: self.data_points.clone(),
            constraints: self.constraints.clone(),
            number_of_clusters: self.number_of_clusters
        };

        // Volvemos a generar un nuevo vecino
        // TODO -- no se si esto afecta demasiado a la eficiencia
        if neighbour.is_valid() == false{
            return self.get_neighbour();
        }

        return neighbour;
    }

    /// Genera una solucion inicial aleatoria, como punto de partida de las busquedas
    // TODO -- no puede dejar clusters vacios
    pub fn generate_random_solution(
        data_points: DataPoints,
        constraints: Vec<Constraint>,
        number_of_clusters: i32
    ) -> Self {
        // Generador de numeros aleatorios
        let mut rng = rand::thread_rng();

        return Self {
            cluster_indexes: (0..data_points.points.len()).map(|_| rng.gen_range(0..number_of_clusters)).collect(),
            data_points,
            constraints,
            number_of_clusters
        };

    }
}
