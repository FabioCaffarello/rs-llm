use langchain_rust::embedding::{Embedder, EmbedderError};
use langchain_rust::schemas::Document;
use regex::Regex;
use std::collections::HashMap;
use futures::executor;

/// Represents the semantic chunker for text, generic over the embedder.
pub struct SemanticChunker<E: Embedder> {
    embedder: E,
    buffer_size: usize,
}

impl<E: Embedder> SemanticChunker<E> {
    pub fn new(embedder: E, buffer_size: usize) -> Self {
        SemanticChunker {
            embedder,
            buffer_size,
        }
    }

    /// Splits text into sentences based on regex, keeping punctuation
    pub fn split_text(&self, text: &str) -> Vec<String> {
        let re = Regex::new(r"([.!?])\s+").unwrap();
        let mut sentences = vec![];
        let mut last_end = 0;

        for cap in re.captures_iter(text) {
            let full_match = cap.get(0).unwrap();

            // Include the sentence and punctuation
            let sentence = &text[last_end..full_match.end()].trim();
            sentences.push(sentence.to_string());

            // Move the starting position for the next sentence
            last_end = full_match.end();
        }

        // If there's any remaining text, add it as the last sentence
        if last_end < text.len() {
            sentences.push(text[last_end..].trim().to_string());
        }

        sentences
    }

    /// Combines sentences based on buffer size
    pub fn combine_sentences(&self, sentences: &[String]) -> Vec<String> {
        let mut combined_sentences = vec![];
        for i in 0..sentences.len() {
            let mut combined_sentence = String::new();
            
            // Add previous buffer size sentences
            for j in (i as isize - self.buffer_size as isize)..i as isize {
                if j >= 0 {
                    combined_sentence.push_str(&sentences[j as usize]);
                    combined_sentence.push(' ');
                }
            }

            // Add the current sentence
            combined_sentence.push_str(&sentences[i]);

            // Add next buffer size sentences
            for j in i + 1..(i + 1 + self.buffer_size) {
                if j < sentences.len() {
                    combined_sentence.push(' ');
                    combined_sentence.push_str(&sentences[j]);
                }
            }

            combined_sentences.push(combined_sentence);
        }
        combined_sentences
    }

    /// Embeds combined sentences using the provided embedder
    pub async fn embed_sentences(&self, sentences: Vec<String>) -> Vec<Vec<f64>> {
        self.embedder.embed_documents(&sentences).await.unwrap()
    }

    /// Create documents from the processed text
    pub fn create_documents(&self, text: String) -> Vec<Document> {
        let sentences = self.split_text(&text);
        let combined_sentences = self.combine_sentences(&sentences);
        let embeddings = executor::block_on(self.embed_sentences(combined_sentences));

        let mut documents = vec![];
        for (sentence, embedding) in sentences.into_iter().zip(embeddings) {
            let mut metadata = HashMap::new();
            metadata.insert("embedding".to_string(), serde_json::json!(embedding));

            let document = Document::new(sentence).with_metadata(metadata);
            documents.push(document);
        }
        documents
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    // Mocking the FastEmbed to avoid calling actual embedding service
    pub struct MockFastEmbed;

    #[async_trait]
    impl Embedder for MockFastEmbed {
        async fn embed_documents(&self, documents: &[String]) -> Result<Vec<Vec<f64>>, EmbedderError> {
            // Mocked embedding: Each sentence gets a simple vector with the length of the sentence
            Ok(documents.iter().map(|doc| vec![doc.len() as f64; 3]).collect())
        }

        async fn embed_query(&self, text: &str) -> Result<Vec<f64>, EmbedderError> {
            Ok(vec![text.len() as f64])
        }
    }

    #[test]
    fn test_split_text() {
        let embedder = MockFastEmbed;
        let chunker = SemanticChunker::new(embedder, 1);
        let text = "Hello world! This is a test.";
        
        let sentences = chunker.split_text(text);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0], "Hello world!");
        assert_eq!(sentences[1], "This is a test.");
    }

    #[test]
    fn test_combine_sentences() {
        let embedder = MockFastEmbed;
        let chunker = SemanticChunker::new(embedder, 1);
        let sentences = vec!["Hello".to_string(), "world!".to_string(), "How are you?".to_string()];
        
        let combined = chunker.combine_sentences(&sentences);
        assert_eq!(combined.len(), 3);
        assert_eq!(combined[0], "Hello world!");
        assert_eq!(combined[1], "Hello world! How are you?");
        assert_eq!(combined[2], "world! How are you?");
    }

    #[tokio::test]
    async fn test_embed_sentences() {
        let embedder = MockFastEmbed;
        let chunker = SemanticChunker::new(embedder, 1);
        let sentences = vec!["Hello world!".to_string(), "This is a test.".to_string()];
        
        let embeddings = chunker.embed_sentences(sentences).await;
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), 3);
        assert_eq!(embeddings[0][0], 12.0); // Length of "Hello world!" is 12
    }

    #[test]
    fn test_create_documents() {
        let embedder = MockFastEmbed;
        let chunker = SemanticChunker::new(embedder, 1);
        let text = "Hello world! This is a test.".to_string();
        
        let documents = chunker.create_documents(text);
        assert_eq!(documents.len(), 2);
        assert_eq!(documents[0].page_content, "Hello world!");
        assert_eq!(documents[1].page_content, "This is a test.");
        
        // Check if embedding is part of the metadata
        assert!(documents[0].metadata.contains_key("embedding"));
    }
}
