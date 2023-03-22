# Weather Mock API

A simple weather mock API implemented in Rust using Actix Web. This API returns a static list of weather forecasts for a given location.

## Table of Contents

- [Weather Mock API](#weather-mock-api)
    - [Table of Contents](#table-of-contents)
    - [Features](#features)
    - [Requirements](#requirements)
    - [Installation](#installation)
    - [Usage](#usage)
    - [Docker](#docker)
        - [Build and Run with Docker](#build-and-run-with-docker)
        - [Stop and Remove the Docker Container](#stop-and-remove-the-docker-container)
    - [Contributing](#contributing)
    - [License](#license)
    - [Support and Issues](#support-and-issues)
    - [Acknowledgments](#acknowledgments)

## Features

- Actix Web framework for building the API
- Tracing for logging and instrumentation
- Serde for serialization and deserialization
- Docker support for easy deployment

## Requirements

- Rust 1.54 or higher
- Docker (optional, for deployment)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/aortizpimentel/weather-mock-api.git
   cd weather-mock-api
   ```
   
2. Build the project:
    ```bash
    cargo build --release
   ```
   
## Usage

1. Run the application:
    ```bash
    cargo run --release
   ```
2. Access the API:
   The API will be available at ```http://localhost:8080/forecasts?lat={latitude}&lon={longitude}```.
Replace ```{latitude}``` and ```{longitude}``` with the desired coordinates.

## Docker
### Build and Run with Docker

1. Build the Docker image:
    ```bash
    docker build -t weather-mock-api .
   ```
2. Run the Docker container:
    ```bash
    docker run -p 8080:8080 --name weather-api weather-mock-api
   ```
3. Access the API:
   
    The API will be available at ```http://localhost:8080/forecasts?lat={latitude}&lon={longitude}```.
Replace ```{latitude}``` and ```{longitude}``` with the desired coordinates.

### Stop and Remove the Docker Container

1. Stop the Docker container:

    ```bash
    docker stop weather-api
   ```
   
2. Remove the Docker container:

    ```bash
    docker rm weather-api
   ```

## Contributing
1. Fork the repository on GitHub
2. Create a new branch for your changes
3. Commit your changes to the new branch
4. Submit a pull request to merge your changes into the main repository

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support and Issues

If you encounter any issues or need help, please open a new [GitHub issue](https://github.com/aortizpimentel/weather-mock-api/issues) in this repository.

## Acknowledgments

- The [Rust](https://www.rust-lang.org/) programming language for its safety and performance features
- The [Actix Web](https://actix.rs/) framework for its extensible and easy-to-use API
- The [Serde](https://serde.rs/) library for its powerful serialization and deserialization capabilities
- The [tracing](https://crates.io/crates/tracing) crate for its excellent logging and instrumentation tools

Happy coding! 🚀