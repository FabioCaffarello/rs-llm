use pyo3::prelude::*;
use tokio::runtime::Runtime;
use langchain_rust::embedding::{Embedder, FastEmbed, InitOptions, EmbeddingModel, TextEmbedding};

/// Struct to handle embedding text
#[pyclass]
pub struct TextEmbedder {
    pub fastembed: FastEmbed,
}

#[pymethods]
impl TextEmbedder {
    /// Constructor for TextEmbedder with default model
    #[new]
    pub fn new() -> PyResult<Self> {
        let fastembed = FastEmbed::try_new().unwrap();
        Ok(TextEmbedder { fastembed })
    }

    /// Constructor with a custom model
    #[staticmethod]
    pub fn new_with_custom_model() -> PyResult<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )
        .unwrap();
    
        let fastembed = FastEmbed::from(model);
        Ok(TextEmbedder { fastembed })
    }

    /// Method to embed text and return embeddings as `Vec<Vec<f32>>`
    pub fn embed_text(&self, text: String) -> PyResult<Vec<Vec<f32>>> {
        let chunks = vec![text];
        let embeddings = Runtime::new().unwrap()
            .block_on(self.fastembed.embed_documents(&chunks))
            .unwrap()
            .into_iter()
            .map(|embedding| embedding.into_iter().map(|v| v as f32).collect())  // Convert f64 to f32
            .collect::<Vec<Vec<f32>>>();
        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_embedder_default() {
        let embedder = TextEmbedder::new().unwrap();
        let text = "This is a test.".to_string();
        let embeddings = embedder.embed_text(text).unwrap();
        assert!(!embeddings.is_empty());
    }

    #[test]
    fn test_text_embedder_with_custom_model() {
        let embedder = TextEmbedder::new_with_custom_model().unwrap();
        let text = "This is another test.".to_string();
        let embeddings = embedder.embed_text(text).unwrap();
        assert!(!embeddings.is_empty());
    }

    #[test]
    fn test_embed_text() {
        let embedder = TextEmbedder::new().unwrap();
        let text = "This is a test.".to_string();
        let embeddings = embedder.embed_text(text).unwrap();
        assert!(!embeddings.is_empty());
        assert_eq!(embeddings[0].len(), 384);  // Assuming 384-dimensional embeddings
    }
}
