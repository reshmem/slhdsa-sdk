#!/usr/bin/env bash
set -euo pipefail

pick_port() {
  local candidate
  for candidate in "${@}"; do
    if ! lsof -iTCP:"${candidate}" -sTCP:LISTEN >/dev/null 2>&1; then
      echo "${candidate}"
      return 0
    fi
    if command -v curl >/dev/null 2>&1; then
      if curl -fsS "http://localhost:${candidate}/status" 2>/dev/null | rg -q "packager-status:running"; then
        echo "${candidate}"
        return 0
      fi
    fi
  done
  echo "${1}"
  return 0
}

PORT="${PORT:-}"
if [[ -z "${PORT}" ]]; then
  PORT="$(pick_port 8081 8082 8083 8084)"
fi

export RCT_METRO_PORT="${PORT}"

if ! command -v curl >/dev/null 2>&1 || ! curl -fsS "http://localhost:${PORT}/status" 2>/dev/null | rg -q "packager-status:running"; then
  echo "Starting Metro on port ${PORT}..."
  npm run start -- --port "${PORT}" >/tmp/slhdsa-metro.log 2>&1 &
  METRO_PID=$!
  trap 'kill ${METRO_PID} >/dev/null 2>&1 || true' EXIT
  for _ in {1..15}; do
    sleep 1
    if command -v curl >/dev/null 2>&1 && curl -fsS "http://localhost:${PORT}/status" 2>/dev/null | rg -q "packager-status:running"; then
      break
    fi
  done
else
  echo "Metro already running on port ${PORT}."
fi

if [[ ! -d "ios/Pods" ]]; then
  echo "Installing CocoaPods..."
  if [[ -f "Gemfile" ]]; then
    bundle install
    (cd ios && bundle exec pod install)
  else
    (cd ios && pod install)
  fi
fi

npm run ios -- --no-packager --port "${PORT}"
