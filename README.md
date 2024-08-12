# Chat gRPC

**Chat gRPC** is a real-time chat service built entirely in Rust using gRPC (Google Remote Procedure Call). It follows a microservice architecture with two main components:  

- **Auth Service**: Handles user authentication and registration.

- **Chat Service**: Manages message broadcasting to all connected clients.

The project is fully asynchronous, ensuring high performance and responsiveness. A Terminal User Interface (TUI) is provided to interact with the service.

# Installation
## Requirements 
- make sure that you have [docker](https://www.docker.com) installed
- make sure that you have [rust](https://www.rust-lang.org/tools/install) installed
- install [bunyan](https://github.com/trentm/node-bunyan) as below (this requires [node](https://nodejs.org/en/download/package-manager) to be installed first)
```sh
    npm install -g bunyan
```

## Set up repo

* Clone the repo with 
```sh
git clone git@github.com:Atheer2104/chat-grpc.git
```

* change the directory with 
```sh
cd chat-grpc
```

Now we will create PostgreSQL and Redis containers using docker, these containers can be stopped and restarted from the docker desktop for further usage.

*  initialize PostgreSQL with 
```sh
auth/scripts/init_db.sh 
```
* Initialize Redis with 
```sh
auth/scripts/init_redis.sh 
```

# Usage 

* Create a separate terminal window and navigate to `cd chat-grpc/auth` and run the auth service with 
```sh
cargo run --release --bin auth-server | bunyan
```

- create a separate terminal window and navigate to `cd chat-grpc/chat` and run chat service with 
```sh
cargo run --release --bin chat-server | bunyan
```

Now you can start one or more clients by having a separate terminal window for each client navigating to `cd chat-grpc/client` and starting the client with 
```bash
cargo run --release --bin chat-client
```

# Technology

Main Technologies used

- [JWT](https://jwt.io) - Used to serve as an access token allowing users to be able to chat
- [PostgreSQL](https://www.postgresql.org) - Used to save user credentials and JWT access token
- [Redis](https://redis.io) - Used to cache JWT auth token
- [Tonic](https://docs.rs/tonic/latest/tonic/) - A rust gRPC library, Used to implement the gRPC functionality
- [Tokio](https://tokio.rs) - A rust Asynchronous runtime, Used to schedule and spawn asynchronous tasks
- [Tracing](https://github.com/tokio-rs/tracing) - Used to write logs asynchronously
- [Ratatui](https://ratatui.rs) -  Used to create Terminal User Interfac
