/// Representa un generador resumido de vecinos
#[derive(Debug)]
pub struct NeighbourGenerator{

    /// El elemento que queremos mover de cluster
    // TODO -- deberia ser u32
    element_index: i32,

    /// El nuevo cluster al que asignamos el elemento
    new_cluster: u32,
}

impl NeighbourGenerator{
    pub fn new(element_index: i32, new_cluster: u32) -> Self{
        return Self {element_index, new_cluster};
    }

    pub fn get_element_index(&self) -> i32{
        return self.element_index;
    }

    pub fn get_new_cluster(&self) -> u32{
        return self.new_cluster;
    }

    /// Genera todos los posibles vecinos, aunque estos no sean validos, dados
    /// el numero de elementos del dataset y el numero de clusters en los que
    /// queremos dividir dichos elementos
    pub fn generate_all_neighbours(number_of_elements: i32, number_of_clusters: i32) -> Vec<Self>{
        let mut neighbours = vec![];

        for current_element in 0..number_of_elements{
            for current_cluster in 0..number_of_clusters{
                neighbours.push(NeighbourGenerator{
                    element_index: current_element,
                    new_cluster: current_cluster as u32,
                });
            }
        }

        return neighbours;
    }
}
