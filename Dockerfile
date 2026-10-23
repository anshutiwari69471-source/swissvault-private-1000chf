FROM rust:1.81 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/swissvault-private-1000chf ./app
COPY --from=builder /app/target/release/swissvault-private-1000chf* ./app 2>/dev/null || true
# fallback if binary name is different
COPY --from=builder /app/target/release/ /app/release/
RUN ls /app/release/ && cp /app/release/swissvault* /app/app 2>/dev/null || cp /app/release/*vault* /app/app 2>/dev/null || find /app/release -maxdepth 1 -type f -executable | head -n 1 | xargs -I {} cp {} /app/app
EXPOSE 8080
ENV PORT=8080
CMD ["./app"]
