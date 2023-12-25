use rand::{self, Rng};

#[derive(Debug, Clone)]
pub struct Neuron{
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<f64>>,
}

impl Neuron{
    pub fn new_from(matrix: Vec<Vec<f64>>) -> Self{
        return Self{
            rows: matrix.len(),
            cols: matrix[0].len(),
            matrix,
        }
    }

    pub fn new_empty(rows: usize, cols: usize) -> Self{
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        for i in 0..rows{
            matrix.push(Vec::new());
            for j in 0..cols{
                matrix[i][j] = 0.0;
            }
        }
        return Self{
            rows,
            cols,
            matrix,
        };
    }

    pub fn new_random(rows: usize, cols: usize) -> Self{
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        for i in 0..rows{
            matrix.push(Vec::new());
            for j in 0..cols{
                matrix[i][j] = rand::thread_rng().gen_range(0.0..1.0);
            }
        }
        return Self{
            rows,
            cols,
            matrix,
        };
    }

    pub fn add(&mut self, other: Neuron){
        if self.rows != other.rows || self.cols != other.cols{
            panic!("These neurons are incapable of being added");
        } else{
            for i in 0..self.rows{
                for j in 0..self.cols{
                    self.matrix[i][j] += other.matrix[i][j];
                }
            }
        }
    }

    pub fn subtract(&mut self, other: Neuron){
        if self.rows != other.rows || self.cols != other.cols{
            panic!("These neurons are incapable of being subtracted");
        } else{
            for i in 0..self.rows{
                for j in 0..self.cols{
                    self.matrix[i][j] -= other.matrix[i][j];
                }
            }
        }
    }

    pub fn cross_product(&self, other: Neuron) -> Neuron{
        if self.cols != other.rows{
            panic!("These neurons are incapable of being multiplied");
        }

        let mut return_neuron = Neuron::new_empty(self.rows, self.cols);

        for i in 0..self.rows{
            for j in 0..other.cols{
                let vec_1 = self.matrix[i].clone();
                let mut vec_2 = Vec::new();
                for k in 0..other.rows{
                    vec_2.push(other.matrix[k][j]);
                }
                for k in 0..self.cols{
                    return_neuron.matrix[i][j] += vec_1[k] * vec_2[k];
                }
            }
        }

        return return_neuron;
    }
}
