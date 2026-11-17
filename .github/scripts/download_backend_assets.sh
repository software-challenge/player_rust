#!/usr/bin/env bash
set -uo pipefail

warn_and_skip() {
  echo "::warning::Failed to download the backend test assets. Example integration tests will not run."
  echo "server_dir=" >> "$GITHUB_OUTPUT"
  echo "random_player_jar=" >> "$GITHUB_OUTPUT"
  exit 0
}

if ! release="$(curl --fail --silent --show-error --retry 3 \
  https://api.github.com/repos/software-challenge/backend/releases/latest 2>/dev/null)"; then
  warn_and_skip
fi

if ! server_url="$(jq -er '.assets[] | select(.name == "software-challenge-server.zip") | .browser_download_url' <<< "$release")"; then
  warn_and_skip
fi

if ! random_player_url="$(jq -er '.assets[] | select((.name | startswith("randomplayer")) and (.name | endswith(".jar"))) | .browser_download_url' <<< "$release")"; then
  warn_and_skip
fi

backend_dir="$RUNNER_TEMP/backend"
mkdir -p "$backend_dir/server"

if ! curl --fail --location --silent --show-error --retry 3 "$server_url" --output "$backend_dir/server.zip"; then
  warn_and_skip
fi

if ! curl --fail --location --silent --show-error --retry 3 "$random_player_url" --output "$backend_dir/randomplayer.jar"; then
  warn_and_skip
fi

if ! unzip -q "$backend_dir/server.zip" -d "$backend_dir/server"; then
  warn_and_skip
fi

server_jar="$(find "$backend_dir/server" -type f -name server.jar -print -quit)"
if [[ -z "$server_jar" ]]; then
  echo "::warning::The latest backend server archive did not contain server.jar. Example integration tests will not run."
  echo "server_dir=" >> "$GITHUB_OUTPUT"
  echo "random_player_jar=" >> "$GITHUB_OUTPUT"
  exit 0
fi

echo "server_dir=$(dirname "$server_jar")" >> "$GITHUB_OUTPUT"
echo "random_player_jar=$backend_dir/randomplayer.jar" >> "$GITHUB_OUTPUT"
