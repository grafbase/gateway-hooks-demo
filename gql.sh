#!/usr/bin/env bash

set -euxo pipefail

before='{"query": "'
query="$1"
after='"}'
shift

curl -X POST 'http://127.0.0.1:5000/graphql' \
    --data "$before$query$after" \
    -H "Content-Type: application/json" \
    "$@" 2>/dev/null |
    jq .
