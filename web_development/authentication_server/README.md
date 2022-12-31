# Authentication Server

Welcome to the Authentication Server project! This project is part of my personal Rust project repository, and is written in Rust using the Actix web framework.

The goal of this project is to provide a lightweight, scalable authentication server that can be used to secure APIs and other applications. One key feature of this server is the ability to encode and decode JSON Web Tokens (JWTs) for use in authentication.

## JWT Tokens

JWTs are a popular way to authenticate users and secure APIs. They are essentially a string that contains a set of claims, which can be any information you want to include. These claims are then signed using a secret key, which allows the server to verify the authenticity of the token.

JWTs have several benefits:

- They are self-contained, meaning they include all the information needed to authenticate a user within the token itself. This makes them easy to use and transport.
- They are stateless, meaning the server does not need to store any information about the user in order to authenticate them. This makes them scalable and easy to use in distributed systems.
- They can be easily passed around, for example in an HTTP header, making them convenient for use in API authentication.

## Actix Web

Actix web is a Rust web framework that is designed to be fast, flexible, and easy to use. It is built on top of the Actix actor system, which allows it to scale well and handle high levels of concurrency.

Some key features of Actix web include:

- A simple, modular design that makes it easy to extend and customize.
- Asynchronous programming support, using the async/await syntax introduced in Rust 1.39.
- Support for WebSockets, HTTP/2, and TLS.
- Built-in support for JSON serialization and deserialization.

## Project Structure

The project is structured as follows:

- The `src` directory contains the source code for the project.
- The `main.rs` file is the entry point for the application and sets up the HTTP server.
- The `models` directory contains the code for encoding and decoding JWTs.
- The `services` directory contains the business logic for the authentication server, including the signup and verify endpoints.
- The `controllers` directory contains the code for handling HTTP requests and responses.
- The `config` directory contains the code for setting up the app.
- The Cargo.toml file is the configuration file for the Rust package manager, Cargo. It specifies the dependencies for the project and other metadata.
- The .env file contains environment variables that are used to configure the application. These include the host and port on which the server should listen, as well as the path to the JWT secret key file.

## Installation

To get started with this project, you'll need to have Rust and Cargo installed on your machine. You can find instructions for doing so on the Rust website.

Once you have Rust and Cargo installed, you can clone this repository and navigate to the project directory. From there, you can run the following command to build and run the server:

```rust
cargo run
```

This will start the server listening on the host and port specified in the .env file. You can then make requests to the server using any HTTP client, such as cURL or Postman.

To sign up a new user, you can make a GET request to the /api/v1/auth/signup endpoint. This will generate a new JWT and return it in the response. To verify a user, you can make a GET request to the /api/v1/auth/verify endpoint, including the JWT in the Authorization header in the format Bearer {JWT}.

Please note that this project is not meant to be used in production. It is intended as a learning exercise and should not be used to secure sensitive data.

I hope you find this project useful and I welcome any feedback or suggestions for improvement. Thank you for checking it out!
