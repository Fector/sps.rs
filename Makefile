.PHONY: up down
up:
	docker-compose -f ./dev/docker-compose.yml up -d

down:
	docker-compose -f ./dev/docker-compose.yml down