# rick-morty

A proxy Server and CLI for the [Rick and Morty API](https://rickandmortyapi.com/documentation/#introduction), written in Rust

## Requirements
* Rust & Cargo
* Git
* Make
* Docker

## Running the Application
* Clone the repository 
* Run `make docker-redis` to start up the Redis Server. Redis is used in implementing caching functionalities.
* Run available tests with `make test`
* Run `make install` to install the CLI.
* Startup the proxy server with `rick-morty start-server`. This also caches the episodes, characters, and locations from the Rick-Morty API
* To create a user, run
```shell
 curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"user1"}' \               
  http://localhost:3002/signup
```
This returns an API Key
* Run `rick-morty get-characters <API_KEY> <CHARACTER_NAME [OPTIONAL]>` to get a character, or all characters if no character input is received.
* Run `rick-morty get-episodes <API_KEY> <CHARACTER_NAME [OPTIONAL]>` to get episodes
* Run `rick-morty get-locations <API_KEY> <CHARACTER_NAME [OPTIONAL]>` to get various locations

## Cleanup
* Run `make clean-docker-redis` to delete the created docker container
