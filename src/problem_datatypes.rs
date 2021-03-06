/// Representa el conjunto de puntos que hay que agrupar
// TODO -- no deberia tener campos publicos
#[derive(Debug)]
pub struct DataPoints{
    pub points: Vec<Point>
}

/// Representa un punto
// TODO -- no deberia tener campos publicos
#[derive(Debug)]
pub struct Point{
    pub coordinates: Vec<f32>
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
    pub first_index: i32,
    pub second_index: i32,
    pub constraint_type: ConstraintType
}
