# Prácticas Metaheurísticas

* Repositorio para desarrollar las prácticas de la asignatura Metaheurísticas

## Librerías de matrices

* `nalgebra`:
    * Muy buena documentación
    * He leído la documentación y creo que me va a permitir realizar todas las operaciones que necesito
        * Descomposiciones de matrices
        * Operaciones con puntos
        * Operaciones usuales geométricas: rotaciones, traslaciones...
    * Más usado que `ndarray`
    * Más fácil de usar, pero puede que más lento
* `ndarray`:
    * Peor documentación, la estándar de un `crate` de Rust
    * Fuertemente inspirado en `numpy`
    * Algunas funciones de álgebra lineal usan otros `crates` como `ndarray-linalg`
    * [Traductor de `numpy` a `ndarray`](https://docs.rs/ndarray/0.12.1/ndarray/doc/ndarray_for_numpy_users/index.html)
    * Más complicado de usar, pero quizás más rápido
