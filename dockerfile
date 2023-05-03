
FROM rust:latest as build

ARG project

# create new project
RUN USER=root cargo new --bin $project

# use project as workdir
WORKDIR $project

# copy manifests
COPY ./Cargo.lock .
COPY ./Cargo.toml .

# build dependencies
RUN cargo build --release & rm ./src/*.rs

# copy  source tree
COPY ./src ./src

# build for release
#RUN rm ./target/release/deps/$project*
RUN cargo build --release
COPY ./model/ /$project/model/

## final base
#FROM rust:latest as app
#
#ARG project
#WORKDIR /app
#
## copy the build artifact from the build stage
#COPY --from=build /$project/ .

# run  binary
CMD ["cargo","run","--release"]
