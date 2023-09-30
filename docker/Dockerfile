# Copyright 2023 Sylk Technologies
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Docker file for plm-cli
FROM rust:latest as cli-build

# create a new empty shell project
RUN USER=root cargo new --bin cli-build
WORKDIR /cli-build
# Install protobuf-compiler
RUN apt-get update && \
    apt-get install -y protobuf-compiler libpq-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Copy workspace files and build
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./protos ./protos
COPY ./plm-core ./plm-core
COPY ./plm-cli ./plm-cli
COPY ./plm-registry ./plm-registry

# this build step will cache dependencies
RUN cargo build --package plm-cli --release
RUN rm -r plm-core/src/*.rs plm-cli/src/*.rs protos plm-registry

# our final base
FROM ubuntu:latest
WORKDIR /cli
# copy the build artifact from the build stage
COPY --from=cli-build /cli-build/target/release/plm ./plm
ENTRYPOINT [ "./plm" ]
