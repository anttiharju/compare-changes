FROM ghcr.io/anttiharju/compare-changes-ci:1.0.0

WORKDIR /workspace
COPY . .

RUN cargo build --target aarch64-apple-darwin
RUN cargo build --target x86_64-unknown-linux-gnu
RUN cargo build --target aarch64-unknown-linux-gnu
