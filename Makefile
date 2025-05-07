.PHONY: docker_up test-curl

docker_up:
	docker compose up --build --force-recreate

test-curl:
	curl localhost:8080/health