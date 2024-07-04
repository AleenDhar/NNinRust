pub mod lib;

use lib::matrix::Matrix;

fn add_1(x:f64)->f64{
    let res= x+1.0;
    return res;
}
fn main() {
    let mut mat = Matrix::random(3,3);
    // let yo = mat.add(Matrix::random(2,2));
    let yo = mat.map(add_1);
    // let yoyo=mat.map();
   
    println!("{:#?} {:#?} ",mat.data, yo.data);
}
