FROM python:3.10-slim

RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"

RUN curl -sSL https://install.python-poetry.org | python3 -
ENV PATH="/root/.local/bin:${PATH}"
RUN poetry config virtualenvs.create false

WORKDIR /app

COPY python_frontend/pyproject.toml python_frontend/poetry.lock* ./
RUN poetry install --no-interaction --no-ansi --no-root

COPY app .

CMD ["python", "app.py"]
