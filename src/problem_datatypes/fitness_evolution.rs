use std::fmt;

/// Representa la evolucion en fitness de los distintos algoritmos de búsqueda
/// iterativa
#[derive(Debug)]
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
}

impl fmt::Display for FitnessEvolution {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Generamos un String con lo que queremos que se muestre por pantalla
        let mut result = format!("{} iterations of the algorithm\n", self.fitness_at_iteration.len());
        for value in &self.fitness_at_iteration{
            result = format!("{}{}, ", result, value);
        }
        result = format!("{}\n", result);

        // Lanzamos esta funcion con nuestro string porque es lo que se espera
        // de esta implementacion para el trait fmt::Display
        return write!(f, "{}", result);
    }
}
