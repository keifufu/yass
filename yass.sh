#!/usr/bin/env bash

# Wayland screenshot example
# Dependencies:
# grim, slurp, imagemagick, swappy, curl, wl-clipboard

TOKEN=TOKEN_HERE
HOST=HOST_HERE
FILENAME=$(date '+%y-%m-%dT%H-%M-%S.png')

grim -g "$(slurp)" - | convert - -shave 1x1 PNG:- | swappy -f - -o - | curl -X PUT $HOST/upload?filename=$FILENAME -H "Authorization: $TOKEN" -H "Content-Type: application/octet-stream" --data-binary "@-" | wl-copy
