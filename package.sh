#!/usr/bin/env bash

set -euo pipefail

mkdir -p ./pkg
cd ./pkg
mkdir -p nodejs/pkg web/pkg
cp ../LICENSE .
cp ../README.md .
cp ../package.dummy.json package.json
cp ../package.nodejs.json nodejs/pkg/package.json
cp ../package.web.json web/pkg/package.json

CARGO_TOML=../Cargo.toml
VERSION=$(tq -r '.package.version' -f "$CARGO_TOML")
AUTHORS=$(tq -r '.package.authors' -f "$CARGO_TOML" | jq -R . | jq -s .)
DESCRIPTION=$(tq -r '.package.description' -f "$CARGO_TOML")
REPOSITORY=$(tq -r '.package.repository' -f "$CARGO_TOML")
LICENSE=$(tq -r '.package.license' -f "$CARGO_TOML")

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
      | .repository = {"type": "git", "url": ("git+" + $repository + ".git")}
      | .license = $license
    ' "$MANIFEST" > $MANIFEST.tmp
  mv $MANIFEST.tmp $MANIFEST
done