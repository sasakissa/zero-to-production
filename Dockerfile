FROM rust:1.47 as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.47 as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Builder stage
FROM rust:1.47 as builder 
WORKDIR app
COPY --from=cacher /app/target/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim as runtime
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRIONMENT production
ENTRYPOINT [ "./zero2prod" ]