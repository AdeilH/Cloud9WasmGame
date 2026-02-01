.PHONY: check-docker build run stop clean help

IMAGE_NAME = league-wasm-game
CONTAINER_NAME = league-wasm-game-local
PORT = 8080

help:
	@echo "Available targets:"
	@echo "  check-docker  : Check if Docker is installed and running"
	@echo "  build         : Build the Docker image for WASM"
	@echo "  run           : Run the Docker container locally on http://localhost:$(PORT)"
	@echo "  stop          : Stop and remove the local Docker container"
	@echo "  clean         : Stop container and remove the Docker image"

check-docker:
	@docker info >/dev/null 2>&1 || (echo "Error: Docker is not running or not installed."; exit 1)
	@echo "Docker is running."

build: check-docker
	@echo "Building WASM app in Docker..."
	docker build -t $(IMAGE_NAME) .

run: build stop
	@echo "Starting container $(CONTAINER_NAME) on port $(PORT)..."
	docker run -d --name $(CONTAINER_NAME) -p $(PORT):$(PORT) -e PORT=$(PORT) $(IMAGE_NAME)
	@echo "Game is accessible at http://localhost:$(PORT)"

stop:
	@echo "Stopping and removing container $(CONTAINER_NAME)..."
	@docker stop $(CONTAINER_NAME) >/dev/null 2>&1 || true
	@docker rm $(CONTAINER_NAME) >/dev/null 2>&1 || true

clean: stop
	@echo "Removing image $(IMAGE_NAME)..."
	@docker rmi $(IMAGE_NAME) >/dev/null 2>&1 || true
