# Rust Discord Bot
Hello, this is my discord bot written in rust. I use this discord bot as a playground for learning new technologies and frameworks. As a result theres a lot of random stuff in this repo

# Running
## Main Exe
The bot can be run using docker. just run `docker compose up` and it should spin up the container. You will need a `.env` file with all of the variables in the `docker-compose.yaml`

## Dependent Services
This bot has a dependency on ollama. The **first** time you set it up run the following commands

#### AMD
```bash
docker run -d --device /dev/kfd --device /dev/dri -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama:rocm
```

#### NVIDA
```bash
docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
```

Once the above commands have been run for the first time you only need to run

```bash
docker start ollama
docker exec -it ollama ollama run gemma3
curl http://localhost:11434/api/pull -d '{
  "model": "gemma3"
}'
```