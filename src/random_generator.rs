use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

lazy_static! {
    static ref RANDOM_GENERATOR: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(0));
}

/// Generador de numeros aleatorios, con una semilla fijada
/// Si fijamos la semilla cada vez que necesitemos un generador de numeros aleatorios, tendremos el
/// problema de que empezariamos por el primer elemento de la sucesion de numeros aleatorios,
/// repitiendo todo el rato los mismos valores
///
/// Usando un static lo que hacemos es que esta estructura emula a un singleton
/// Para que sea thread_safe, tenemos que usar un Mutex
///
/// No tiene campos porque con lo que vamos a jugar es con el lazy_static!
pub struct RandomGenerator{}

impl RandomGenerator{
    /// Devuelve un entero en un rango dado
    pub fn gen_range(&self, lower: i32, upper: i32) -> i32 {
        return RANDOM_GENERATOR.lock().unwrap().gen_range(lower..upper);
    }

    /// Modificamos el generador de numeros aleatorios con la nueva semilla
    /// Al ser estatico, afecta a todos los elementos que llamen a funciones de esta
    /// estructura
    /// Asi solo contruimos una vez el nuevo generador de numeros, y evitamos repetir la misma
    /// secuencia aleatoria una y otra vez
    pub fn set_seed(&self, new_seed: u64){
        *RANDOM_GENERATOR.lock().unwrap() = StdRng::seed_from_u64(new_seed);
    }
}
