#!/usr/bin/env bash
set -euo pipefail

APP_NAME="smn_server_sbs"
DEPLOY_DIR="/opt/smn_server_sbs"
BIN_NAME="smn_server_sbs"
PID_FILE="${DEPLOY_DIR}/${APP_NAME}.pid"

echo "=== Stopping existing service (if any) ==="
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo "Killing process $OLD_PID"
        kill "$OLD_PID"
        sleep 1
    fi
    rm -f "$PID_FILE"
fi

echo "=== Building project ==="
cargo build --release

echo "=== Deploying to ${DEPLOY_DIR} ==="
sudo mkdir -p "$DEPLOY_DIR"
sudo cp "target/release/${BIN_NAME}" "${DEPLOY_DIR}/${BIN_NAME}"
sudo rm -rf "${DEPLOY_DIR}/res"
sudo cp -r "res" "${DEPLOY_DIR}/res"

sudo chmod +x "${DEPLOY_DIR}/${BIN_NAME}"

echo "=== Starting service ==="
cd "$DEPLOY_DIR"

nohup "./${BIN_NAME}" > "${DEPLOY_DIR}/stdout.log" 2> "${DEPLOY_DIR}/stderr.log" &
NEW_PID=$!

echo $NEW_PID | sudo tee "$PID_FILE" >/dev/null

echo "Service started with PID $NEW_PID"
echo "Working dir: $DEPLOY_DIR"
