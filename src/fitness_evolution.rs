use std::fmt;
use npy::to_file;
use serde::{Serialize};

/// Representa la evolucion en fitness de los distintos algoritmos de búsqueda
/// iterativa
#[derive(Debug, Serialize)]
pub struct FitnessEvolution{
    fitness_at_iteration: Vec<f64>,
}

impl FitnessEvolution{
    /// Crea una instancia vacia del FitnessEvolution
    pub fn new() -> Self{
        return Self{fitness_at_iteration: vec![]};
    }

    /// Genera una instancia a partir del vector de datos
    pub fn from_vec(fitness_at_iteration: Vec<f64>) -> Self{
        return Self{fitness_at_iteration};
    }

    /// Añade un nuevo valor del fitness, o lo que es lo mismo, una iteracion mas
    /// a nuestra representacion
    pub fn add_iteration(&mut self, fitness: f64){
        self.fitness_at_iteration.push(fitness);
    }

    /// Devuelve los valores del fitness
    pub fn get_fitness_at_iteration(&self) -> Vec<f64>{
        return self.fitness_at_iteration.clone();
    }

    /// Guardamos el resultado en un fichero de Numpy. Esto porque vamos a mostrar graficas usando
    /// scripts de python, trabajando con numpy
    pub fn save_as_numpy_file(&self, file_path: &str) -> Option<()>{
        let result = npy::to_file(file_path, self.fitness_at_iteration.clone());

        // No quiero tanto control, asi que paso de Result a Option
        match result {
            Ok(_) => return Some(()),
            Err(e) => {
                eprintln!("[Err] No se pudo guardar el archivo npy");
                eprintln!("Codigo de error: {}", e);
                return None;
            }
        }
    }
}

// Para poder mostrar por pantalla el FitnessEvolution
impl fmt::Display for FitnessEvolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Numero de valores que vamos a mostrar en una fila
        let elements_at_row = 4;

        // Generamos un String con lo que queremos que se muestre por pantalla
        let mut result = format!("{} iterations of the algorithm\n", self.fitness_at_iteration.len());
        for value in &self.fitness_at_iteration{
            for _ in 0..elements_at_row{
                result = format!("{}{}, ", result, value);
            }

            result = format!("{}{}", result, "\n");
        }
        result = format!("{}\n", result);

        // Lanzamos esta funcion con nuestro string porque es lo que se espera
        // de esta implementacion para el trait fmt::Display
        return write!(f, "{}", result);
    }
}
