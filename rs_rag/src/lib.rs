use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn calculate_pi(n_terms: usize) -> PyResult<f64> {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;
    for _ in 0..n_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    Ok(pi)
}

// FIXME: Implement this function
#[pyfunction]
fn create_embeddings(n_terms: usize) -> PyResult<f64> {
    let mut pi = 0.0;
    Ok(pi)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_llm(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    m.add_function(wrap_pyfunction!(create_embeddings, m)?)?;
    Ok(())
}
