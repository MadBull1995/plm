FROM rust:latest as registry-build

# create a new empty shell project
RUN USER=root cargo new --bin registry-build
WORKDIR /registry-build
# Install protobuf-compiler
RUN apt-get update && \
    apt-get remove -y libpq5 && \
    apt-get install -y protobuf-compiler libpq-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Copy workspace files and build
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./protos ./protos
COPY ./plm-core ./plm-core
COPY ./plm-registry ./plm-registry
COPY ./plm-cli ./plm-cli

# this build step will cache dependencies
RUN cargo build --package plm-registry --release
RUN rm -r plm-core/src/*.rs plm-registry/src/*.rs protos plm-cli

# our final base
FROM ubuntu:latest
WORKDIR /registry
RUN apt-get update && \
    apt install -y python3-psycopg2


ENV REGISTRY_CONFIG="./config.json"
# ENV PROTOT_GRPC_PORT=44880
# copy the build artifact from the build stage
COPY --from=registry-build /registry-build/target/release/plm-registry ./plm-registry
COPY ./data/registry/config.docker.json ./config.json
# CMD ./registry init --data-host ${PROTOT_REDIS_HOST} --grpc-port ${PROTOT_GRPC_PORT}
CMD ./plm-registry ${REGISTRY_CONFIG}
