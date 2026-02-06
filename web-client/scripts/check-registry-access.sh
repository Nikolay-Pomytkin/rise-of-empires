#!/usr/bin/env bash
set -euo pipefail

REGISTRY_URL="${1:-https://registry.npmjs.org/pixi.js}"

echo "Checking Bun registry reachability: ${REGISTRY_URL}"
TMP_ERR=$(mktemp)
HTTP_STATUS=$(curl -I -sS -o /dev/null -w "%{http_code}" "$REGISTRY_URL" 2>"$TMP_ERR" || true)
CURL_ERR=$(cat "$TMP_ERR")
rm -f "$TMP_ERR"

if [[ "$HTTP_STATUS" == "200" ]]; then
  echo "OK: registry reachable"
  exit 0
fi

if [[ "$HTTP_STATUS" == "403" ]] || [[ "$CURL_ERR" == *"response 403"* ]]; then
  echo "ERROR: proxy/firewall denied registry access (HTTP 403)."
  echo "Ask for npm registry allowlisting or update proxy policy."
  exit 1
fi

echo "ERROR: registry request failed (HTTP ${HTTP_STATUS})."
if [[ -n "$CURL_ERR" ]]; then
  echo "$CURL_ERR"
fi
exit 1
