#!/bin/bash

case "$1" in
    start)
        echo "Starting MCP authentication server..."
        python3 examples/mcp-authentication/auth_server.py &

        pushd examples/mcp-authentication/keycloak
        docker compose up -d
        popd
        ;;
    stop)
        pkill -f "examples/mcp-authentication/auth_server.py" 2>/dev/null || true
        pushd examples/mcp-authentication/keycloak
        docker compose down
        popd
        ;;
    *)
        echo "Usage: $0 {start|stop}"
        exit 1
        ;;
esac
