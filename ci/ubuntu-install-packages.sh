#!/bin/sh

# This script gets run in weird environments that have been stripped of just
# about every inessential thing. In order to keep this script versatile, we
# just install 'sudo' and use it like normal if it doesn't exist. If it doesn't
# exist, we assume we're root. (Otherwise we ain't doing much of anything
# anyway.)
if ! command -V sudo; then
  apt-get update
  apt-get install -y --no-install-recommends sudo
fi
sudo apt-get update
sudo apt-get remove -y libpq5
sudo apt-get install -y --no-install-recommends \
  protobuf-compiler libpq-dev
sudo apt install -y python3-psycopg2