DOCKER_COMMAND                        = docker
DOCKER_REDIS_CONTAINER_NAME           = redis
DOCKER_REDIS_CONTAINER_LOCAL_PORT     = 6399

# Exported variables
export CACHED_REDIS_CONNECTION_STRING = redis://127.0.0.1:$(DOCKER_REDIS_CONTAINER_LOCAL_PORT)
export REDIS_HOSTNAME=redis://127.0.0.1
export IS_TLS=redis

# Starts a Redis server using `DOCKER_COMMAND`
docker-redis: docker-status
	@echo [$@]: Starting Redis container...
	-$(DOCKER_COMMAND) run --rm --name $(DOCKER_REDIS_CONTAINER_NAME) \
 		-p $(DOCKER_REDIS_CONTAINER_LOCAL_PORT):6379 -d redis

docker-status:
	@echo [$@]: Checking the Docker engine
	@docker info > /dev/null || (>&2 echo 'Is the Docker engine running?' && exit 42)
# Formats  rust code
fmt:
	@echo [$@]: Formatting code...
	cargo fmt

# Cleans all generated artifacts and deletes all docker containers
clean: clean-docker clean-cargo clean-docker-redis

# Runs `cargo clean`
clean-cargo:
	@echo [$@]: Removing cargo artifacts...
	cargo clean

# Removes all docker containers
clean-docker: clean-docker-$(DOCKER_REDIS_CONTAINER_NAME)

# Removes a docker container with the given name
clean-docker-redis:
	@echo [$@]: Removing container called $*...
	$(DOCKER_COMMAND) rm -f $*