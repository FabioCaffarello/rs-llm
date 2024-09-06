use pyo3::prelude::*;

mod embeddings;
mod semantic_chunker;

use embeddings::TextEmbedder;



/// A Python module implemented in Rust.
#[pymodule]
fn rs_llm(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    _m.add_class::<TextEmbedder>()?;
    Ok(())
}
