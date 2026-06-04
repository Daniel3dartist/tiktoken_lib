FROM debian:bookworm-slim

WORKDIR /usr/src/tiktoken_lib

RUN apt-get update && apt-get install -y \
    build-essential \
    gcc \
    g++ \
    make \
    cmake \
    git \
    libreadline-dev \
    libssl-dev \
    libsqlite3-dev \
    libbz2-dev \
    liblzma-dev \
    libffi-dev \
    libncursesw5-dev \
    libreadline-dev \
    libsqlite3-dev

COPY . .

CMD [ "bash" ]