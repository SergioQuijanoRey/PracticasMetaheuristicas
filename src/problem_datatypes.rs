/// Representa el conjunto de puntos que hay que agrupar
#[derive(Debug)]
pub struct DataPoints{
    points: Vec<Point>
}

impl DataPoints{
    pub fn new(points: Vec<Point>) -> Self{
        return Self{points};
    }
}

/// Representa un punto
// TODO -- no deberia tener campos publicos
#[derive(Debug)]
pub struct Point{
    coordinates: Vec<f32>
}

impl Point{
    pub fn new(coordinates: Vec<f32>) -> Self{
        return Self{coordinates};
    }
}

#[derive(Debug)]
pub enum ConstraintType{
    MustLink,
    CannotLink,
}

/// Estructura de datos que representa una restriccion
/// Una restriccion viene dada por los dos indices de los elementos que se
/// restringen y el tipo de restriccion
// TODO -- pasar esta estructura de datos a un hash para tener acceso directo
#[derive(Debug)]
pub struct Constraint{
    first_index: i32,
    second_index: i32,
    constraint_type: ConstraintType
}

impl Constraint{
    pub fn new(first_index: i32, second_index: i32, constraint_type: ConstraintType) -> Self{
        return Self{first_index, second_index, constraint_type};
    }
}

/// Estructura que representa una solucion del problema
///
/// La solucion viene representada como un vector de indices
/// En dicho vector, la posicion i-esima indica el cluster al que pertenece el i-esimo
/// punto del conjunto de datos
pub struct Solution{
    cluster_indexes: Vec<i32>,
    number_of_clusters: i32,
    number_of_elements: i32,
}

/// TODO -- falta por implementar
impl Solution{
    pub fn new(cluster_indexes: Vec<i32>, number_of_clusters: i32, number_of_elements: i32) -> Self{
        return Self{cluster_indexes, number_of_clusters, number_of_elements};
    }

    /// Comprueba si la solucion es valida o no
    // TODO -- ¿tiene que ser publico?
    pub fn is_valid(&self) -> bool{
        return false;
    }

    /// Calcula el valor de fitness de la solucion
    pub fn fitness(&self) -> f32{
        return 0.0;
    }

    /// Devuelve un vecino de la solucion
    // TODO -- no puede dejar clusters vacios
    pub fn get_neighbour(&self) -> Self{
        return Self{cluster_indexes: vec![], number_of_clusters: 0, number_of_elements: 0};
    }

    /// Genera una solucion inicial aleatoria, como punto de partida de las busquedas
    // TODO -- no puede dejar clusters vacios
    pub fn generate_random_solution(number_of_clusters, number_of_elements) -> Self{
        return Self{cluster_indexes: vec![], number_of_clusters: 0, number_of_elements: 0};
    }
}
