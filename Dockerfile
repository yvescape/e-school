# Image unique : le binaire Rust sert l'API et le frontend compilé.

# 1. Frontend
FROM node:24-alpine AS frontend
WORKDIR /build/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# 2. Backend
FROM rust:1-slim-bookworm AS backend
WORKDIR /build/backend
COPY backend/ ./
RUN cargo build --release --locked -p app --bin app

# 3. Exécution
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 e_school
WORKDIR /app
COPY --from=backend /build/backend/target/release/app /app/e_school
COPY --from=frontend /build/frontend/dist /app/public
ENV BIND_ADDR=0.0.0.0:8080 STATIC_DIR=/app/public LOG_FORMAT=json
USER e_school
EXPOSE 8080
CMD ["/app/e_school"]
