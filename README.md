
# Rust Based SSH Client

This project is a basic SSH client written in Rust. It connects to a server, authenticates a session, and executes a command. The result of the command is then printed. It uses the ssh2 crate for SSH connections and std::net::TcpStream for TCP connections.

## Features

- Establishes a TCP connection to the SSH server based on the provided IPv4 address.
- Creates an SSH session.
- Authenticates the SSH session using a username and password.
- Executes a command on the server and prints the result.

### Replace placeholders

- Replace ipv4, username and password accordingly.


## Run Locally

Clone the project

```bash
  git clone https://github.com/anujchourasia15/rust-ssh-client.git
```

Go to the project directory

```bash
  cd rust-ssh-client
```

Run

```bash
  cargo run
```

