use std::io;

fn main() {
    println!("Welcome to Rust NN inference!");

    let matrix = vec![
        vec![1,2],
        vec![3,4],
    ];

    println!("Enter the vector components");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let vector: Vec<i32> = input
    .trim()
    .split_whitespace()
    .map(|x| x.parse().expect("Invalid input"))
    .collect();

    // Assert vector length is equal to matrix columns
    assert_eq!(vector.len(), matrix[0].len());

    // Multiply matrix and vector
    let mut result = vec![0; matrix.len()];
    for i in 0..matrix.len() {
        for j in 0..vector.len() {
            result[i] += matrix[i][j] * vector[j];
        }
    }

    // Apply relu to result

    for i in 0..result.len() {
        if result[i] < 0 {
            result[i] = 0;
        }
    }

    // dot product with unit vector

    let mut dot_product = 0;
    for i in 0..result.len() {
        dot_product += result[i];
    }
    
    println!("Inference: {:?}", dot_product);

}

// Embedd a 2 dimensional neural network

