use rand::Rng;
use ndarray::Array;

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

    /// Dado un cluster (representado por el entero que los identifica), calcula
    /// la distancia intracluster en la solucion actual
    // TODO -- comprobar que no estemos dividiendo por cero, ya sea con un result
    // o con un panic!
    pub fn get_intra_cluster_distance(&self, cluster: i32) -> f32{
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
    fn get_points_in_cluster(&self, cluster: i32) -> Vec<&Point>{
        let mut cluster_points = vec![];

        for (index, curr_cluster) in self.cluster_indexes.iter().enumerate(){
            if *curr_cluster == cluster{
                cluster_points.push(&self.data_points.points[index]);
            }
        }

        return cluster_points;
    }
}
