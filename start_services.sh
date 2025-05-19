#!/usr/bin/env bash

set -e

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

# Run Ollama with gemma3
echo "Running Ollama with gemma3 model..."
docker exec -it ollama ollama run gemma3

# Start docker compose services
echo "Starting Docker Compose services..."
docker compose up
