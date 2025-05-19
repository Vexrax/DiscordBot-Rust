#!/usr/bin/env bash

set -e

# === Configurable variables ===
MODEL="llama4"

# Check if 'ollama' container exists
if ! docker container inspect ollama > /dev/null 2>&1; then
    echo "Creating and starting Ollama container for the first time..."
    docker run -d \
        --device /dev/kfd \
        --device /dev/dri \
        -v ollama:/root/.ollama \
        -p 11434:11434 \
        --name ollama \
        ollama/ollama:rocm
else
    echo "Ollama container already exists. Starting it..."
    docker start ollama
fi

# Wait a bit to ensure ollama is ready
sleep 2

# Pull (download) the model
echo "Pulling model: $MODEL"
docker exec ollama ollama pull "$MODEL"

# Start docker compose with build
echo "Building and starting Docker Compose services..."
docker compose up --build