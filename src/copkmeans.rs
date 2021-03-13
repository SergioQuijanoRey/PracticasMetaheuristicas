use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::problem_datatypes::NeighbourGenerator;
use crate::problem_datatypes::Point;
use std::process::exit;
use rand::seq::SliceRandom; // Para hacer shuffle de un vector



/// Genera los centroides de forma aleatoria
/// Como los puntos del problema estan normalizados en el intervalo [0, 1]^2, los
/// centroides aleatorios estarán en dicho intervalo
/// El cluster i-esimo tiene centroide el punto i-esimo del vector
fn generate_random_centroids(number_of_clusters: i32, point_dimension: i32) -> Vec<Point>{
    let mut centroids = vec![];

    for _ in 0..number_of_clusters{
        // Genero un punto aleatorio que sera el centroide actual
        let current_centroid = Point::random_point(point_dimension);

        // Lo añado al vector de centroides
        centroids.push(current_centroid);
    }

    return centroids;
}

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
pub fn run(data_points: DataPoints, constraints: Constraints, number_of_clusters: i32, seed: i32) -> Solution{
    // Necesitamos generar numeros aleatorios para recorrer los puntos en un
    // orden aleatorio
    let mut rng = rand::thread_rng();

    // TODO -- borrar estas indicaciones
    // Pasos para desarrollar el algoritmo:
    // 1. Tomar number_of_clusters centroides aleatorios
    // 2. Mientras los centroides no cambien
    //      2.1 Recorremos los elementos en orden aleatorio
    //      2.2 Por cada elemento, lo asignamos a un cluster
    //          2.2.1 Preferencia por numero de restricciones violadas
    //          2.2.2 Desempate por distancia al centroide
    //      2.3 Recalcular los nuevos centroides
    //

    // Numero de coordenadas que componen cada uno de los puntos
    // Necesario para saber cuantas coordenadas debe tener nuestros centroides aleatorios
    let point_dimension = match data_points.point_dimension(){
        Some(dimension) => dimension as i32,
        None => {
            eprintln!("[Err] la estructura DataPoints no tiene puntos de los que obtener su dimension");
            eprintln!("No se puede continuar con la busqueda greedy");
            exit(-1);
        }
    };

    // Tomamos los centroides aleatorios
    let mut current_centroids = generate_random_centroids(number_of_clusters, point_dimension);

    // Solucion inicial que en cuanto iteremos una vez vamos a sobreescribir
    // Ahora solo nos interesa considerar los indices de los clusters
    // Notar que esta solucion no es valida porque deja todos los clusters
    // menos uno vacio
    let mut current_cluster_indixes = vec![0; data_points.len() as usize];

    // Iteramos hasta que los centroides no cambien
    let mut centroids_have_changed = true;
    while centroids_have_changed == true {
        // Realizamos una nueva asignacion de clusters
        let mut new_cluster_indixes = vec![0; data_points.len() as usize];

        // Recorremos aleatoriamente los puntos para irlos asignando a cada cluster
        // TODO -- hay que hacer la asignacion con cabeza
        let mut point_indexes: Vec<u32> = (0..data_points.len() as u32).collect();
        point_indexes.shuffle(&mut rng);
        for index in point_indexes{
            new_cluster_indixes[index as usize] = select_best_cluster() as u32;

            // TODO -- borrar esto
            new_cluster_indixes[index as usize] = index % number_of_clusters as u32;
        }

        // Calculamos los nuevos centroides
        // Para ello, generamos una solucion para usar sus funcionalidades
        // TODO -- separarlo en otra funcion
        let new_cluster_indixes: Vec<i32> = new_cluster_indixes.into_iter().map(|x| x as i32).collect();
        let tmp_solution = Solution::new(new_cluster_indixes.clone(), data_points.clone(), constraints.clone(), number_of_clusters, seed);
        let mut new_centroids = vec![];
        for cluster in 0 .. number_of_clusters{
            // Tomamos los puntos que pertenecen a este cluster
            let cluster_points = tmp_solution.get_points_in_cluster(cluster);

            // Añadimos el centroide de ese conjunto de puntos
            new_centroids.push(Point::calculate_centroid(cluster_points));
        }

        // Comprobamos si los centroides han cambiado
        // TODO -- separarlo en una funcion
        centroids_have_changed = false;
        for index in 0 .. new_centroids.len(){
            if (new_centroids[index] == current_centroids[index]) == false{
                println!("Punto {:?} es distinto a punto {:?}", new_centroids[index], current_centroids[index]);
                centroids_have_changed = true;
                break;
            }
        }

        // Cambiamos a la nueva asignacion de clusters y los nuevos centroides
        current_cluster_indixes = new_cluster_indixes;
        current_centroids = new_centroids;
    }


    // Convierto los tipos del vector de clusters
    let current_cluster_indixes = current_cluster_indixes.into_iter().map(|x| x as i32).collect();

    // Devuelvo la solucion a partir del vector de asignacion de clusters
    return Solution::new(current_cluster_indixes, data_points, constraints, number_of_clusters, seed);
}

// Dado un punto y una configuracion de puntos actual, elige el mejor cluster posible
fn select_best_cluster() -> i32{
    return 13;
}
