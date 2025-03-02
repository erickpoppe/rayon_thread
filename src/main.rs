/*Prueba de rayon
 Erick Poppe
 2025
 */


use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}

fn main() {
    let slice: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("La suma es {:?}", sum_of_squares(slice));
}