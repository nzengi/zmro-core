// src/utils/math.rs

/// Computes the Euclidean distance between two points in n-dimensional space.
///
/// # Arguments
///
/// * `point1` - A slice representing the first point.
/// * `point2` - A slice representing the second point.
///
/// # Returns
///
/// * `f64` - The Euclidean distance between the two points.
pub fn euclidean_distance(point1: &[f64], point2: &[f64]) -> f64 {
    let sum_of_squares: f64 = point1.iter()
        .zip(point2.iter())
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum();
    sum_of_squares.sqrt()
}
