#!/usr/bin/env bash

set -euo pipefail

cd ./pkg
cp ../LICENSE .
cp ../README.md .
cp ../package.dummy.json package.json
cp ../package.nodejs.json nodejs/pkg/package.json
cp ../package.web.json web/pkg/package.json

VERSION=$(cargo pkgid | awk -F'[#@]' '{print $NF}')
for MANIFEST in package.json nodejs/pkg/package.json web/pkg/package.json; do
  sed -i "s/__VERSION__/$VERSION/g" "$MANIFEST"
done