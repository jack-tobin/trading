FROM python:3.10-slim-bullseye

# Install Rust and required dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# Install maturin for building Rust extensions
RUN pip install --no-cache-dir maturin[patchelf]
COPY engine/ ./engine/
RUN maturin build --release --manifest-path engine/Cargo.toml

# Copy the Python application
COPY app/ ./app/
COPY pyproject.toml poetry.lock* ./

# Install Python dependencies and the built Rust extension
RUN pip install --no-cache-dir poetry && \
    pip install engine/target/wheels/*.whl && \
    poetry config virtualenvs.create false && \
    poetry install --no-interaction --no-ansi

EXPOSE 8050

CMD ["python", "-m", "app.app"]
