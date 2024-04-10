FROM rustlang/rust:nightly-bullseye as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/content /app/content
COPY --from=builder /app/public /app/public

COPY --from=builder /app/target/release/my-blog /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

WORKDIR /app

ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="leptos-blog"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["/app/my-blog"]
