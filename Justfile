default:
    just --list

dev:
    docker-compose up --build

undev:
    docker-compose down
