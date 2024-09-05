# Stage 1: Build stage
FROM python:3.11-slim AS builder

# Install necessary build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    gcc \
    libssl-dev \
    libffi-dev \
    curl \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust and Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Poetry
RUN curl -sSL https://install.python-poetry.org | python3 - && \
    mkdir -p /opt/poetry
ENV PATH="/root/.local/bin:${PATH}"

# Verify Poetry installation
RUN poetry --version

# Copy the project files
WORKDIR /app
COPY pyproject.toml poetry.lock Cargo.toml poetry.toml ./
COPY rs_rag /app/rs_rag
COPY rs_llm /app/rs_llm

# Install project dependencies and build the Rust-Python wheel using maturin
RUN poetry install --no-root --no-dev && \
    poetry run maturin develop

# Stage 2: Runtime stage
FROM python:3.11-slim AS runtime

# Install runtime dependencies (if needed)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libffi-dev \
    curl \
    gcc \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Poetry directly for the user
RUN curl -sSL https://install.python-poetry.org | python3 -
ENV PATH="/root/.local/bin:${PATH}"

# Verify Poetry installation in the runtime stage
RUN poetry --version

# Set PYTHONPATH to point to the virtual environment
ENV PYTHONPATH="/app/.venv/lib/python3.11/site-packages:${PYTHONPATH}"

# Copy the application code and Python environment from the builder stage
COPY --from=builder /app /app

WORKDIR /app

# Run the application with poetry run to ensure the correct environment is used
ENTRYPOINT ["poetry", "run", "python", "/app/rs_llm/main.py"]
