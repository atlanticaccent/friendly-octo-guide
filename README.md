# Truelayer Coding Challenge

A Pokemon description retrieval and description translation API, written in Rust.

## Installation

### Dependencies

- Rust 1.56+
- (If running outside of container) OpenSSL
- (Optional) Docker
- (Optional) Docker Compose

#### Installing Rust

Follow the instructions for your platform as detailed here: [Install Rust](https://www.rust-lang.org/tools/install)

## Building a local binary

Building is as simple as running:

`cargo build`

For a release build, run:

`cargo build --release`

## Building with Dockerfile

The simplest approach for building and running with Docker is with Docker Compose, however you can also manually build the container yourself.

To do so, run the following:

`docker build -t {your chosen container name} .`

## Running tests

To run tests, run:

`cargo test`

## Running the server locally

To run the server locally, simply execute:

`cargo run`

or

`cargo run --release`

## Running with Docker or Docker Compose

To run with docker, run the following, substituting in the name you gave the container when you built it earlier and the port number you would like to access the server on:

`docker run -p {chosen port}:8080 --rm {container name}`

The even simpler option is to run the application with Compose, which runs the compilation, container building and running steps for you:

`docker compose up`

## Notes

### Logging and errors

Given more time, I would have happily improved the logging and error messaging aspects of the application. Currently there is no runtime logging on the server, and error responses to clients are generally unhelpful. More informative and ideally correct error messages would be helpful for clients, so that they can understand what issues the server might have, and what steps they can take to account for them.

### Cache tuning

In production, it would almost certainly be an improvement to fine tune the cache properties, given an understanding of the demands placed on the API and other variables. Most certainly, some Pokemon will be more popular than others, and so be requested more often, but there is also the question of which of the two endpoints would be more popular?

There is also the fact that the Funtranslations API has much more extreme limitations than Pokeapi - it would then make sense to increase the weighting of cached translation values even if they aren't hit as often, but then by how much?

The current maximum cache size is based on the approximate number of Pokemon that currently exist (excluding new Pokemon that may appear in the upcoming Pokemon Legends game), however shrinking or expanding the cache to reflect demand and operating conditions may be better.

There is also the fact that the cache is in memory only, and so will not persist between application runs. However, if a disk persisted cache were more desirable, it may be more prudent to build a partial mirror of the entirety of Pokeapi and possible Funtranslation responses, thus alleviating the network cost on the external APIs entirely - an in memory cache would still aid in response times however as retrieval from memory will always be faster than over the network or from disk.
