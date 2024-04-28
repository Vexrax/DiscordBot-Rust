# Rust Discord Bot
Hello, this is my discord bot written in rust. I use this discord bot as a playground for learning new technologies and frameworks. As a result theres a lot of random stuff in this repo

# Running
## Main Exe
The bot can be run using docker. just run `docker compose up` and it should spin up the container. You will need a `.env` file with all of the variables in the `docker-compose.yaml`

## Dependent Services
This bot has a dependency on ollama, spin up the ollama container by running

#### AMD
```bash
docker run -d --device /dev/kfd --device /dev/dri -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama:rocm
```

#### NVIDA
```bash
docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
```

and then running

```bash
docker exec -it ollama ollama run llama3
```