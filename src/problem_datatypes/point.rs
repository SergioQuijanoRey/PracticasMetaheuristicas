// Para fijar la semilla de numeros aleatorios
use rand::Rng;

// Modulo para manejar arrays
use ndarray::Array;

/// Representa un punto
#[derive(Debug, PartialEq)]
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

    /// Genera un punto aleatorio cuyas coordenadas se encuentran siempre en
    /// el intervalo [0, 1]
    pub fn random_point(number_of_coordinates: i32) -> Self{
        let mut rng = rand::thread_rng();

        // Array de ceros
        let coordinates: ndarray::Array1<f32> = ndarray::Array1::zeros(number_of_coordinates as usize);

        // En cada elemento colocamos un valor aleatorio
        let coordinates = coordinates.mapv(|_| rng.gen_range(0.0 .. 1.0));

        return Self{coordinates};

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
    pub fn calculate_centroid(points: &Vec<&Self>) -> Self{
        // Condicion de seguridad
        if points.len() == 0{
            panic!("No se puede calcular el centroide de un conjunto vacio de puntos")
        }

        // Array de zeros con el mismo shape que el primer punto que le pasemos
        let mut sum_point = ndarray::Array1::zeros(points[0].coordinates.len());

        // Calculamos el centroide
        for point in points{
            sum_point = sum_point + &point.coordinates;
        }
        sum_point = sum_point / points.len() as f32;

        return Self{coordinates: sum_point};
    }

    /// Dado un conjunto de puntos, calcula la maxima distancia entre dos de ellos
    pub fn max_distance_among_two(points: &Vec<Point>) -> f32{
        let mut max_dist = 0.0;

        for i in 0 .. points.len(){
            for j in i .. points.len(){
                let curr_dist = Self::distance(&points[i], &points[j]);

                if curr_dist > max_dist{
                    max_dist = curr_dist;
                }
            }
        }

        return max_dist;

    }

    /// Devuelve la dimension del punto
    /// Es decir, el numero de coordenadas del punto
    pub fn dimension(&self) -> usize {
        return self.coordinates.len();
    }
}
