#!/usr/bin/env bash

set -euo pipefail

cd ./pkg
cp ../LICENSE .
cp ../README.md .
cp ../package.dummy.json package.json
cp ../package.nodejs.json nodejs/pkg/package.json
cp ../package.web.json web/pkg/package.json

CARGO_TOML=../Cargo.toml
VERSION=$(tomlq -r '.package.version' "$CARGO_TOML")
AUTHORS=$(tomlq -r '.package.authors[]' "$CARGO_TOML" | jq -R . | jq -s .)
DESCRIPTION=$(tomlq -r '.package.description' "$CARGO_TOML")
REPOSITORY=$(tomlq -r '.package.repository' "$CARGO_TOML")
LICENSE=$(tomlq -r '.package.license' "$CARGO_TOML")

for MANIFEST in package.json nodejs/pkg/package.json web/pkg/package.json; do
  sed -i "s/__VERSION__/$VERSION/g" "$MANIFEST"
  jq \
    --argjson collaborators "$AUTHORS" \
    --arg description "$DESCRIPTION" \
    --arg repository "$REPOSITORY" \
    --arg license "$LICENSE" \
    -r \
    '
      .collaborators = $collaborators
      | .description = $description
      | .repository = {"type": "git", "url": $repository}
      | .license = $license
    ' "$MANIFEST" > $MANIFEST.tmp
  mv $MANIFEST.tmp $MANIFEST
done