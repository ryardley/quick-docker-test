This tests using QUIC to connect two containers in docker swarm. 

## Usage 

## Initialize Swarm

Initialize swarm on your local machine if you have not yet done so.

```
docker swarm init
```

## Build the docker image

Build the docker image locally.

```
./build.sh
```

This will build the Dockerfile

## Deploy to the local swarm cluster

Deploy the local image to your local swarm.

```
./deploy.sh

Updating service quic_server (id: zpaasdu4zku1lfl9ymups8nf8)
image localrepo/quic-app:latest could not be accessed on a registry to record
its digest. Each node will access localrepo/quic-app:latest independently,
possibly leading to different nodes running different
versions of the image.

Updating service quic_client (id: 1nr5qhtheje2565slzp9g1s7c)
image localrepo/quic-app:latest could not be accessed on a registry to record
its digest. Each node will access localrepo/quic-app:latest independently,
possibly leading to different nodes running different
versions of the image.
```

Don't worry about the warnings as the local node still has access to the image

## Inspect the logs

```
./inspect.sh
```

This should give you output like the following which shows what has happened:

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

## Explanation

This code implements a basic QUIC client-server test using the Quinn library. Here's a detailed breakdown:
The program can run in two modes (controlled by the ROLE environment variable):
Server Mode (ROLE=server):

1. Generates a self-signed certificate for "localhost"
1. Creates a QUIC server endpoint listening on port 4433 (all interfaces)
1. Waits for incoming connections
1. When a connection arrives, accepts it and reads incoming data
1. Prints received messages to stdout

Client Mode (default):

1. Creates a QUIC client endpoint with disabled certificate verification
1. Attempts to connect to a server named "server" on port 4433 (This uses docker's hostname service discovery)
1. Once connected, sends "Hello!" message
1. Keeps the client alive with an infinite sleep loop (so that the binary is not restarted by swarm)
