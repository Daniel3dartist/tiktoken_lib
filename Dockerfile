FROM rust:1-bookworm

RUN apt-get update && apt-get install -y \
    scons \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
COPY . .

RUN cargo build --release && scons test=1

CMD ["bash"]
