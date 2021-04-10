use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ConstraintType {
    MustLink,
    CannotLink,
}

/// Estructura de datos que representa las restricciones del problema
/// Usamos un hashmap por motivos de eficiencia la hora de guardar y acceder a los datos
/// Una restriccion viene dada por los dos indices de los elementos que se restringen
/// y el tipo de restriccion
/// Es lo mismo la restriccion sobre indices (i, j) que la restriccion sobre indices (j, i)
/// Las restricciones MustLink del tipo (i, i) no se almacenan al ser triviales
/// Siempre vamos a trabajar con restricciones del tipo (i, j) con i < j para
/// que sea mas facil de programar algunas funcionalidades
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
            let (smaller, bigger) = Self::order_pair(first_index, second_index);
            self.data.insert((smaller, bigger), constraint_type);
        }
    }

    // Comprueba si tenemos el elemento dado por los indices
    // A mano se comprueba que (a, b) == (b, a) a la hora de mirar las claves
    pub fn has_element(&self, first_index: i32, second_index: i32) -> bool{
        return self.data.contains_key(&(first_index, second_index)) || self.data.contains_key(&(second_index, first_index));
    }

    pub fn get_constraint(&self, first_index: i32, second_index: i32) -> Option<&ConstraintType>{
        // Tenemos el elemento
        if self.has_element(first_index, second_index) {

            // Guardamos siempre los datos en la forma (i, j) con i < j
            let (smaller, bigger) = Self::order_pair(first_index, second_index);
            return self.data.get(&(smaller, bigger));

        }

        // No hay una restriccion entre los dos elementos pasados como parametros
        return None;
    }

    pub fn get_data(&self) -> &HashMap<(i32, i32), ConstraintType>{
        return &self.data;
    }

    /// Toma dos enteros y devuelve el par ordenado en orden ascendente
    fn order_pair(first: i32, second: i32) -> (i32, i32){
        if first < second {
            return (first, second);
        }else{
            return (second, first);
        }

    }
}

#[cfg(test)]
mod test{
    use crate::problem_datatypes::Constraints;
    use crate::problem_datatypes::ConstraintType;

    #[test]
    fn test_has_element(){
        let mut constraints = Constraints::new();
        constraints.add_constraint(0, 1, ConstraintType::MustLink);
        constraints.add_constraint(3, 4, ConstraintType::CannotLink);

        let calc_inside = constraints.has_element(0, 1);
        let exp_inside = true;
        assert_eq!(calc_inside, exp_inside);

        let calc_inside = constraints.has_element(1, 0);
        let exp_inside = true;
        assert_eq!(calc_inside, exp_inside);

        let calc_inside = constraints.has_element(3, 4);
        let exp_inside = true;
        assert_eq!(calc_inside, exp_inside);

        let calc_inside = constraints.has_element(4, 3);
        let exp_inside = true;
        assert_eq!(calc_inside, exp_inside);

        let calc_inside = constraints.has_element(1, 3);
        let exp_inside = false;
        assert_eq!(calc_inside, exp_inside);
    }

    #[test]
    fn test_correct_returned_constraints(){
        let mut constraints = Constraints::new();
        constraints.add_constraint(0, 1, ConstraintType::MustLink);
        constraints.add_constraint(3, 4, ConstraintType::CannotLink);

        let calc_returned_constraint = constraints.get_constraint(0, 1);
        match calc_returned_constraint{
            Some(ConstraintType::MustLink) => (),
            Some(ConstraintType::CannotLink) => panic!("Should return MustLink, CannotLink returned"),
            None => panic!("Should return MustLink, None returned"),
        }

        let calc_returned_constraint = constraints.get_constraint(1, 0);
        match calc_returned_constraint{
            Some(ConstraintType::MustLink) => (),
            Some(ConstraintType::CannotLink) => panic!("Should return MustLink, CannotLink returned"),
            None => panic!("Should return MustLink, None returned"),
        }

        let calc_returned_constraint = constraints.get_constraint(3, 4);
        match calc_returned_constraint{
            Some(ConstraintType::MustLink) => panic!("Should return CannotLink, MustLink returned"),
            Some(ConstraintType::CannotLink) => (),
            None => panic!("Should return CannotLink, None returned"),
        }

        let calc_returned_constraint = constraints.get_constraint(4, 3);
        match calc_returned_constraint{
            Some(ConstraintType::MustLink) => panic!("Should return CannotLink, MustLink returned"),
            Some(ConstraintType::CannotLink) => (),
            None => panic!("Should return CannotLink, None returned"),
        }

        let calc_returned_constraint = constraints.get_constraint(1, 3);
        match calc_returned_constraint{
            Some(ConstraintType::MustLink) => panic!("Should return None, MustLink returned"),
            Some(ConstraintType::CannotLink) => panic!("Should return None, CannotLink returned"),
            None => (),
        }
    }

}
