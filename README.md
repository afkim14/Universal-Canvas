# Universal Canvas

Universal Canvas is an online shared canvas where users can paint individual pixels. The back-end system is developed in Rust, making us of websockets and concurrency. The front-end is made with Javascript's p5js library.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

What things you need to install the software and how to install them

```
curl https://sh.rustup.rs -sSf | sh
```

### Running

Step by step instruction to get the project running.

First, run the rust server.

```
cargo build
cargo run
```

Access the server through your desired client menthod by accessing the ip address. By default, it runs on localhost.

```
localhost:8080
```
