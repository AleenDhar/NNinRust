use core::panic;
use std::clone;
use std::iter;

use rand::{thread_rng, Rng};

#[derive(Debug,Clone)]
pub struct Matrix{
    pub rows:usize,
    pub cols:usize,
    pub data:Vec<Vec<f64>>
}
impl IntoIterator for Matrix {
    type Item = Vec<f64>;
    type IntoIter = std::vec::IntoIter<Vec<f64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl FromIterator<Vec<f64>> for Matrix {
    fn from_iter<I: IntoIterator<Item = Vec<f64>>>(iter: I) -> Self {
        let data: Vec<Vec<f64>> = iter.into_iter().collect();
        Matrix::from(data)
    }
}
impl Matrix{
 
    pub fn from(data:Vec<Vec<f64>>)->Matrix{
        Matrix{
            rows:data.len(),
            cols:data[0].len(),
            data:data
        }
    }
    pub fn zeros(rows:usize, cols:usize)->Matrix{
        Matrix{
            rows,
            cols,
            data: vec![vec![0.0;cols];rows],
        }  
    }
    pub fn random(rows:usize,cols:usize)-> Matrix{
        let mut rng= thread_rng();
        let mut res = Matrix::zeros(rows, cols);
        for i in 0..rows{
            for j in 0..cols{
                res.data[i][j] = rng.gen::<f64>() *2.0 -1.0 ;
            }
        }
        res
    }
    pub fn mul(&mut self, mat:Matrix)->Matrix{
        if self.cols !=mat.rows {
            panic!("the matix dimensions are not suitabel for multiplicaltion");
        }
        let mut res = Matrix::zeros(self.cols, mat.rows);
        
        for i in 0..self.rows{
            for j in 0..mat.cols{
                let mut sum =0.0;
                for k in 0..self.cols{
                    sum+= self.data[i][k]*mat.data[k][i];
                }
                res.data[i][j]= sum
            }
        }
        res
    }
    pub fn add(&mut self, mat:Matrix) -> Matrix{
        if self.cols != mat.cols || self.rows != mat.rows {
            panic!("dont have the same dimensions")
        }
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data[i][j]=self.data[i][j]+mat.data[i][j]
            }
        }
        res
    }
    pub fn dot_mul(&mut self, mat:Matrix) -> Matrix{
        if self.cols != mat.cols || self.rows != mat.rows {
            panic!("dont have the same dimensions")
        }
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data[i][j]=self.data[i][j]*mat.data[i][j]
            }
        }
        res
    }
    pub fn sub(&mut self, mat:Matrix) -> Matrix{
        if self.cols != mat.cols || self.rows != mat.rows {
            panic!("dont have the same dimensions")
        }
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data[i][j]=self.data[i][j]-mat.data[i][j]
            }
        }
        res
    }
    // the map method need an into_iter but cant use it
    pub fn map(&self, func: fn(f64)->f64)->Matrix{
        let data = self.data.clone();
        Matrix::from(data).clone().into_iter().map(|row| row.into_iter().map(|value| func(value)).collect()).collect()
    }
    pub fn t(&self)-> Matrix{
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data[j][i]=self.data[i][j]
            }
        }
        res
    }
}