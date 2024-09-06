# rs-llm

`rs-llm` is a project designed to create efficient Python bindings for Rust-based large language models (LLMs) using [Maturin](https://github.com/PyO3/maturin). This repository allows users to combine Rust's high-performance capabilities with Python’s flexibility to build robust LLM-based applications, especially for Retrieval-Augmented Generation (RAG) tasks.

## Features

- **Rust-Python Integration**: Seamlessly bridge Rust and Python using Maturin, enabling high-performance computations within a Python-friendly interface.
- **Retrieval-Augmented Generation (RAG)**: Utilize a framework for RAG tasks, powered by the performance of Rust, making document retrieval and question-answering highly efficient.
- **Modular Pipelines**: Customize and extend the architecture for use cases such as document processing, semantic search, and intelligent query answering.

## Components

- **`rs_llm`**: This is the Python application that serves as the entry point for interacting with the Rust libraries. It handles tasks like loading documents, interacting with LLMs, and performing RAG-based queries using Python.
  
- **`rs_rag`**: This contains the Rust libraries that power the Retrieval-Augmented Generation logic. It includes functionalities like embeddings generation, semantic splitting, and efficient document processing. These libraries are accessed through Python bindings, providing the best of both Rust and Python in a single solution.

## Getting Started

### Prerequisites

- **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).
- **Python 3.10+**: Ensure Python is installed.

### Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/FabioCaffarello/rs-llm.git
   cd rs-llm
   ```

2. **Build the Python bindings**:
   ```bash
   maturin develop
   ```

3. **Install Dependencies**:
   ```bash
   poetry install
   ```

4. **Run the Application**: 
   Build and run the project using Docker:
   ```bash
   make run
   ```

## Usage

You can use `rs-llm` to perform various tasks involving LLMs, such as:

- **Processing large documents** and performing advanced retrieval tasks with semantic splitting.
- **Generating responses** using embedded models for various use cases such as answering queries from large text corpora.
- **Optimizing RAG tasks** by combining Rust’s performance with Python’s flexibility.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
