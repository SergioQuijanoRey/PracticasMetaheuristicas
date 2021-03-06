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
