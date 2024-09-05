PROJECT_NAME=rs-llm

build:
	maturin develop
	poetry build

image:
	docker build -t fabiocaffarello/$(PROJECT_NAME):latest -f ./Dockerfile .

run: image
	docker-compose -f docker-compose.yml up -d

logs:
	docker-compose logs -f $(PROJECT_NAME)

stop:
	docker-compose down -v --remove-orphans

cleanup:
	@containers=$$(docker ps -q -a); \
	if [ -n "$$containers" ]; then \
		docker rm -f $$containers; \
	else \
		echo "No containers to remove"; \
	fi

purge-images:
	@docker images --filter "dangling=true" -q | xargs -r docker rmi