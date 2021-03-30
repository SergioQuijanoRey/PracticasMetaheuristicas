use std::boxed::Box;
use std::error::Error;

// Para no tener que poner el path del modulo de los datatypes
use crate::problem_datatypes::{Constraints, ConstraintType, DataPoints, Point};

/// Toma un fichero de datos y los parsea a la estructura de datos correspondiente
pub fn parse_data_file_to_struct(data_path: &str) -> Result<DataPoints, Box<dyn Error>> {
    // Tiene que ser mutable para poder iterar
    // El proceso de tomar el siguiente elemento se considera una mutacion de
    // la variable
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false) // Nuestro fichero no tiene headers
        .from_path(data_path)?;

    // Vector de puntos que vamos a construir
    let mut points: Vec<Point> = vec![];

    for current_line in reader.records() {
        // Unwrap el result
        let current_line = current_line?;

        // Mapear StringRecord a un vector de floats
        let data: Vec<&str> = current_line.iter().collect();
        let data: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();

        // Si algun elemento no se puede parsear, se devuelve un error
        // En otro caso, me tomo los datos
        let data: Result<Vec<f64>, _> = data.into_iter().map(|x| x.parse::<f64>()).collect();
        let data = data?;

        // Añadir el punto al vector de puntos
        let point = Point::from_vec(data);
        points.push(point)
    }

    return Ok(DataPoints::new(points));
}

/// Toma un fichero de restricciones y los parsea a la correspondiente estructura de datos
pub fn parse_constraints_file_to_struct(constraint_file_path: &str) -> Result<Constraints, Box<dyn Error>> {
    // El vector de restricciones que vamos a construir
    let mut constraints = Constraints::new();

    // Tiene que ser mutable para poder iterar
    // El proceso de tomar el siguiente elemento se considera una mutacion de
    // la variable
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false) // Nuestro fichero no tiene headers
        .from_path(constraint_file_path)?;

    for (index, current_line) in reader.records().enumerate() {
        // Unwrap el result
        let current_line = current_line?;

        // Mapear StringRecord a un vector de enteros
        let data: Vec<&str> = current_line.iter().collect();
        let data: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();

        // Si algun elemento no se puede parsear, se devuelve un error
        // En otro caso, me tomo los datos
        let data: Result<Vec<i32>, _> = data.into_iter().map(|x| x.parse::<i32>()).collect();
        let data = data?;

        // Recorremos el vetor de restricciones y añadimos las restricciones que nos encontremos
        for (second_index, value) in data.into_iter().enumerate() {
            if value == 1 {
                constraints.add_constraint(index as i32, second_index as i32, ConstraintType::MustLink);
            }

            if value == -1 {
                constraints.add_constraint(index as i32, second_index as i32, ConstraintType::CannotLink);
            }
        }
    }

    return Ok(constraints);
}
