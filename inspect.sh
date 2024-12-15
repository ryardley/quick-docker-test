#!/usr/bin/env bash
echo ""
echo "================================="
echo "           QUIC SERVER "
echo "================================="

docker service logs quic_server

echo ""
echo "================================="
echo "           QUIC CLIENT "
echo "================================="
docker service logs quic_client
