#!/usr/bin/env bash
set -euo pipefail

logs="$RUNNER_TEMP/example-integration-logs"
mkdir -p "$logs"
server_pid=
random_player_pid=

cleanup() {
  status=$?
  for pid in "$random_player_pid" "$server_pid"; do
    if [[ -n "$pid" ]]; then
      kill "$pid" 2>/dev/null || true
      wait "$pid" 2>/dev/null || true
    fi
  done
  if [[ "$status" -ne 0 ]]; then
    for log in "$logs"/*.log; do
      if [[ -f "$log" ]]; then
        echo "::group::$(basename "$log")"
        cat "$log"
        echo "::endgroup::"
      fi
    done
  fi
}
trap cleanup EXIT

(
  cd "$SERVER_DIR"
  java -Dfile.encoding=UTF-8 -Dlogback.configurationFile=logback.xml \
    -jar server.jar --port 13051
) > "$logs/server.log" 2>&1 &
server_pid=$!

server_ready=false
for _ in {1..30}; do
  if ! kill -0 "$server_pid" 2>/dev/null; then
    echo "::error::The backend test server exited before becoming ready."
    exit 1
  fi
  if grep -q "Listening on port 13051 for incoming connections." "$logs/server.log"; then
    server_ready=true
    break
  fi
  sleep 1
done
if [[ "$server_ready" != true ]]; then
  echo "::error::The backend test server did not listen on port 13051 within 30 seconds."
  exit 1
fi

mapfile -t examples < <(
  cargo metadata --no-deps --format-version 1 |
    jq -r --arg manifest "$GITHUB_WORKSPACE/Cargo.toml" \
      '.packages[] | select(.manifest_path == $manifest) | .targets[] | select(.kind | index("example")) | .name' |
    sort
)
if (( ${#examples[@]} == 0 )); then
  echo "::error::Cargo did not report any example targets to test."
  exit 1
fi

for example in "${examples[@]}"; do
  echo "::group::Testing example: $example"
  timeout --signal=TERM 240s java -jar "$RANDOM_PLAYER_JAR" --port 13051 \
    > "$logs/random-player-$example.log" 2>&1 &
  random_player_pid=$!

  timeout --signal=TERM 240s cargo run --locked --all-features --example "$example" -- --port 13051 \
    > "$logs/$example.log" 2>&1
  wait "$random_player_pid"
  random_player_pid=

  cat "$logs/$example.log"
  echo "::endgroup::"
done
