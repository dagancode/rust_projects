# Stage 1 - Build
FROM rust:1.86 AS builder
ENV SQLX_OFFLINE=true
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2 - Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/property_analysis .
CMD [ "./property_analysis" ]