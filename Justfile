# Lists all available commands
default:
    just --list

# Runs the containerized services
dev:
    docker-compose up --build

# Stops the containerized services
undev:
    docker-compose down

# Builds all Mate Components for Development
build:
    cargo b --all
