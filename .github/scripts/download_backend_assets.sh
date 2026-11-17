#!/usr/bin/env bash
set -uo pipefail

warn_and_skip() {
  echo "::warning::Failed to download the backend test assets. Example integration tests will not run."
  echo "backend_available=false" >> "$GITHUB_OUTPUT"
  echo "server_dir=" >> "$GITHUB_OUTPUT"
  echo "random_player_jar=" >> "$GITHUB_OUTPUT"
  exit 0
}

fail_backend_assets() {
  local http_code="$1"
  echo "::error::The backend assets request failed with HTTP ${http_code}."
  echo "backend_available=false" >> "$GITHUB_OUTPUT"
  echo "server_dir=" >> "$GITHUB_OUTPUT"
  echo "random_player_jar=" >> "$GITHUB_OUTPUT"
  exit 1
}

check_http_status() {
  local http_code="$1"
  if [[ "$http_code" =~ ^4[0-9][0-9]$ ]]; then
    fail_backend_assets "$http_code"
  fi
  if [[ ! "$http_code" =~ ^2[0-9][0-9]$ ]]; then
    warn_and_skip
  fi
}

token="${GH_TOKEN:-${GITHUB_TOKEN:-}}"
api_headers=(
  -H "Accept: application/vnd.github+json"
  -H "X-GitHub-Api-Version: 2022-11-28"
)
if [[ -n "$token" ]]; then
  api_headers+=( -H "Authorization: Bearer $token" )
fi

release_response="$(curl --location --silent --show-error --retry 3 "${api_headers[@]}" \
  -w '\n%{http_code}' https://api.github.com/repos/software-challenge/backend/releases/latest 2>/dev/null)"
release_status="${release_response##*$'\n'}"
release="${release_response%$'\n'*}"
check_http_status "$release_status"

if ! server_url="$(jq -er '.assets[] | select(.name == "software-challenge-server.zip") | .browser_download_url' <<< "$release")"; then
  warn_and_skip
fi

if ! random_player_url="$(jq -er '.assets[] | select((.name | startswith("randomplayer")) and (.name | endswith(".jar"))) | .browser_download_url' <<< "$release")"; then
  warn_and_skip
fi

backend_dir="$RUNNER_TEMP/backend"
mkdir -p "$backend_dir/server"

server_http_code="$(curl --location --silent --show-error --retry 3 -w '%{http_code}' "$server_url" --output "$backend_dir/server.zip")"
check_http_status "$server_http_code"

random_player_http_code="$(curl --location --silent --show-error --retry 3 -w '%{http_code}' "$random_player_url" --output "$backend_dir/randomplayer.jar")"
check_http_status "$random_player_http_code"

if ! unzip -q "$backend_dir/server.zip" -d "$backend_dir/server"; then
  warn_and_skip
fi

server_jar="$(find "$backend_dir/server" -type f -name server.jar -print -quit)"
if [[ -z "$server_jar" ]]; then
  echo "::warning::The latest backend server archive did not contain server.jar. Example integration tests will not run."
  echo "backend_available=false" >> "$GITHUB_OUTPUT"
  echo "server_dir=" >> "$GITHUB_OUTPUT"
  echo "random_player_jar=" >> "$GITHUB_OUTPUT"
  exit 0
fi

echo "backend_available=true" >> "$GITHUB_OUTPUT"
echo "server_dir=$(dirname "$server_jar")" >> "$GITHUB_OUTPUT"
echo "random_player_jar=$backend_dir/randomplayer.jar" >> "$GITHUB_OUTPUT"
