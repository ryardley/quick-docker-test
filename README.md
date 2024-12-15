# QUIC Container Communication Test

This project demonstrates QUIC protocol communication between two containers in a single-node Docker Swarm environment.

## Getting Started

First, initialize your local Docker Swarm if you haven't already:

```
docker swarm init
```

Build the Docker image locally using:

```
./build.sh
```

Deploy the services to your Swarm cluster:

```
./deploy.sh
```

Note: You can safely ignore the registry access warnings - the local node will still function properly.

## Monitoring Communication

View the interaction between containers using:

```
./inspect.sh
```

You should see output like this:

```
=================================
           QUIC SERVER
=================================
quic_server.1.qofwltpxqala@nixos    | [1734259405] Server starting...
quic_server.1.qofwltpxqala@nixos    | [1734259405] Server listening on :4433
quic_server.1.qofwltpxqala@nixos    | Server received connection attempt
quic_server.1.qofwltpxqala@nixos    | Server established connection
quic_server.1.qofwltpxqala@nixos    | Got: Hello!

=================================
           QUIC CLIENT
=================================
quic_client.1.x4t19ket4za3@nixos    | [1734259408] Client starting...
quic_client.1.x4t19ket4za3@nixos    | [1734259408] Client attempting connection to 10.0.5.2:4433
quic_client.1.x4t19ket4za3@nixos    | [1734259408] Client connected
quic_client.1.x4t19ket4za3@nixos    | [1734259408] Client sending message
quic_client.1.x4t19ket4za3@nixos    | [1734259408] Client finished sending
```

## How It Works

The application uses the Quinn library and operates in two modes controlled by the ROLE environment variable:
Server Mode (ROLE=server) generates a self-signed certificate for localhost, listens on port 4433, and handles incoming connections by displaying received messages.

Client Mode connects to the container running the "quic_server" service on port 4433 (using Docker Swarm's DNS service discovery), sends a "Hello!" message, and maintains an active connection through an infinite loop to prevent Swarm from restarting the container.
