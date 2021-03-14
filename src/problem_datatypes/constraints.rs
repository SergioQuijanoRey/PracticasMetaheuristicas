use std::collections::HashMap;

#[derive(Debug)]
pub enum ConstraintType {
    MustLink,
    CannotLink,
}

// TODO -- sobrescribir el tipo de dato (i32, i32) para que sea lo mismo (1, 2) que (2, 1)
/// Estructura de datos que representa las restricciones del problema
/// Usamos un hashmap por motivos de eficiencia la hora de guardar y acceder a los datos
/// Una restriccion viene dada por los dos indices de los elementos que se restringen
/// y el tipo de restriccion
#[derive(Debug)]
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

    pub fn get_data(&self) -> &HashMap<(i32, i32), ConstraintType>{
        return &self.data;
    }
}
