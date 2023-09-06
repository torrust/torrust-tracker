#!/bin/bash

echo "Building docker image ..."

docker build --target release --tag torrust-tracker:release --file Containerfile .
