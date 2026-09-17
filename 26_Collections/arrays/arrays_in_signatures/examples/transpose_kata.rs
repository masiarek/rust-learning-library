//! Kata solution: transpose a 3x3 matrix, then every square matrix, then any.
//!
//!   rustc --edition 2024 transpose_kata.rs -o /tmp/tk && /tmp/tk

/// The first draft: two index loops. Row i of the input becomes column i.
fn transpose_3x3(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

/// Every square size: N comes from the argument.
fn transpose<const N: usize>(m: [[i32; N]; N]) -> [[i32; N]; N] {
    std::array::from_fn(|i| std::array::from_fn(|j| m[j][i]))
}

/// Any shape: R rows of C become C rows of R. The type says so.
fn transpose_rect<T: Copy, const R: usize, const C: usize>(m: [[T; C]; R]) -> [[T; R]; C] {
    std::array::from_fn(|c| std::array::from_fn(|r| m[r][c]))
}

fn main() {
    let matrix = [
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("1. Two index loops");
    println!("   {:?}", transpose_3x3(matrix));

    println!();
    println!("2. from_fn inside from_fn, for any N");
    println!("   {:?}", transpose(matrix));
    println!("   2x2: {:?}", transpose([[101, 102], [201, 202]]));
    println!("   twice is the identity: {}", transpose(transpose(matrix)) == matrix);

    println!();
    println!("3. Not square: [[T; 3]; 2] in, [[T; 2]; 3] out");
    let wide = [['a', 'b', 'c'], ['d', 'e', 'f']];
    let tall = transpose_rect(wide);
    println!("   {wide:?}");
    println!("   {tall:?}");
    println!("   {} -> {}", std::any::type_name_of_val(&wide), std::any::type_name_of_val(&tall));

    println!();
    println!("4. {{:#?}} prints one number per line, which is why a test's");
    println!("   failure message for a matrix is long:");
    println!("{:#?}", transpose([[101, 102], [201, 202]]));
}
