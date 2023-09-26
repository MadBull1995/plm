#!/bin/bash

# @bufbuild/protoc-gen-validate
./clone_and_publish bufbuild/protoc-gen-validate:./:main

# @envoyproxy/envoy
./clone_and_publish envoyproxy/envoy:api/:main

# @googleapis/googleapis
./clone_and_publish googleapis/googleapis:./:master

# @grpc/grpc
./clone_and_publish exclude=src/proto/math,src/proto/grpc/testing,src/proto/grpc/gcp grpc/grpc:src/proto/:master