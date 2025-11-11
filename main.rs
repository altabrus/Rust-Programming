// Rust Lab: Basic Functions

// -----------------------------------------------------------
// Function: quadratic
// Purpose:  Computes a quadratic expression a + b*x + c*x^2
// Parameters:
//   a, b, c, x : unsigned integers (u32)
// Returns: u32 - the computed value of a + b*x + c*x^2
// -----------------------------------------------------------
fn quadratic(a: u32, b: u32, c: u32, x: u32) -> u32 {
    a + b * x + c * x * x
}

// -----------------------------------------------------------
// Function: scale_vector
// Purpose:  Scales a 2D vector (x, y) by a scalar value s
// Parameters:
//   s : f64 - scalar value
//   v : (f64, f64) - vector to be scaled
// Returns: (f64, f64) - scaled vector (s*x, s*y)
// -----------------------------------------------------------
fn scale_vector(s: f64, v: (f64, f64)) -> (f64, f64) {
    (s * v.0, s * v.1)
}

// -----------------------------------------------------------
// Function: dot_product
// Purpose:  Computes the dot product of two 2D vectors
// Parameters:
//   v1, v2 : (f64, f64)
// Returns: f64 - dot product = v1.x * v2.x + v1.y * v2.y
// -----------------------------------------------------------
fn dot_product(v1: (f64, f64), v2: (f64, f64)) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1
}

// -----------------------------------------------------------
// Function: hadamard_product
// Purpose:  Computes the Hadamard product (element-wise)
// Parameters:
//   v1, v2 : (f64, f64)
// Returns: (f64, f64) - result = (v1.x * v2.x, v1.y * v2.y)
// -----------------------------------------------------------
fn hadamard_product(v1: (f64, f64), v2: (f64, f64)) -> (f64, f64) {
    (v1.0 * v2.0, v1.1 * v2.1)
}

// -----------------------------------------------------------
// Test cases
// -----------------------------------------------------------
fn main() {
    println!("quadratic(2, 3, 4, 5) = {}", quadratic(2, 3, 4, 5));
    println!("scale_vector(5.0, (3.0, 4.0)) = {:?}", scale_vector(5.0, (3.0, 4.0)));
    println!("dot_product((1.0, 2.0), (3.0, 4.0)) = {}", dot_product((1.0, 2.0), (3.0, 4.0)));
    println!("hadamard_product((1.0, 2.0), (3.0, 4.0)) = {:?}", hadamard_product((1.0, 2.0), (3.0, 4.0)));
}
