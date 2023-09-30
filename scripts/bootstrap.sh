#!/bin/bash
# @open-telemetry/opentelemetry-proto
# ./scripts/clone_and_publish.sh open-telemetry/opentelemetry-proto:./:main

# @grafeas/grafeas
# ./scripts/clone_and_publish.sh grafeas/grafeas:./:master

# @googleapis/googleapis
./scripts/clone_and_publish.sh googleapis/googleapis:./:master

# @bufbuild/protoc-gen-validate
# ./scripts/clone_and_publish.sh bufbuild/protoc-gen-validate:./:main

# @cncf/udpa
# ./scripts/clone_and_publish.sh cncf/udpa:udpa/:main

# @cncf/xds
# ./scripts/clone_and_publish.sh cncf/xds:./:main

# @envoyproxy/envoy
# ./scripts/clone_and_publish.sh exclude=api/contrib envoyproxy/envoy:api/:main

# @googleapis/googleapis
# ./clone_and_publish googleapis/googleapis:./:master

# # @grpc/grpc
# ./clone_and_publish exclude=src/proto/math,src/proto/grpc/testing,src/proto/grpc/gcp grpc/grpc:src/proto/:master