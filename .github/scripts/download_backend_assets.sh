#!/usr/bin/env bash
set -euo pipefail

release="$(curl --fail --silent --show-error --retry 3 \
  https://api.github.com/repos/software-challenge/backend/releases/latest)"
server_url="$(jq -er '.assets[] | select(.name == "software-challenge-server.zip") | .browser_download_url' <<< "$release")"
random_player_url="$(jq -er '.assets[] | select((.name | startswith("randomplayer")) and (.name | endswith(".jar"))) | .browser_download_url' <<< "$release")"

backend_dir="$RUNNER_TEMP/backend"
mkdir -p "$backend_dir/server"
curl --fail --location --silent --show-error --retry 3 "$server_url" \
  --output "$backend_dir/server.zip"
curl --fail --location --silent --show-error --retry 3 "$random_player_url" \
  --output "$backend_dir/randomplayer.jar"
unzip -q "$backend_dir/server.zip" -d "$backend_dir/server"

server_jar="$(find "$backend_dir/server" -type f -name server.jar -print -quit)"
if [[ -z "$server_jar" ]]; then
  echo "::error::The latest backend server archive did not contain server.jar."
  exit 1
fi

echo "server_dir=$(dirname "$server_jar")" >> "$GITHUB_OUTPUT"
echo "random_player_jar=$backend_dir/randomplayer.jar" >> "$GITHUB_OUTPUT"
