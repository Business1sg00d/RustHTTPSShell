FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
apt-get install -y pkg-config libclang-dev curl wget git build-essential unzip gcc-mingw-w64 python3 && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
