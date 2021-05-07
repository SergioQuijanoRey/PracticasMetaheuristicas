/// Representa un resultado que devolvemos en forma Resultado + evaluaciones del fitness consumidas
/// en el proceso
pub struct FitnessEvaluationResult<T>{
    result: T,
    iterations_consumed: u32,
}

impl<T> FitnessEvaluationResult<T>{
    pub fn new(result: T, iterations_consumed: u32) -> Self{
        return Self{result, iterations_consumed}
    }

    pub fn get_result(&self) -> &T{
        return &self.result;
    }

    pub fn get_iterations_consumed(&self) -> u32{
        return self.iterations_consumed;
    }

    pub fn get_mut_result(&mut self) -> &mut T{
        return &mut self.result;
    }

    pub fn get_mut_iterations_consumed(&mut self) -> u32{
        return self.iterations_consumed;
    }
}
