# 1. Этап сборки (Builder)
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Копируем манифесты и исходники
COPY Cargo.toml Cargo.lock ./
COPY static ./static
COPY src ./src

# Собираем релизный бинарник
RUN cargo build --release

# 2. Финальный образ
FROM debian:bookworm-slim

WORKDIR /app

# Устанавливаем сертификаты (для внешних HTTPS запросов)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Создаем папку под постоянные данные (SQLite база)
RUN mkdir -p /app/data

# Забираем скомпилированный бинарник
COPY --from=builder /app/target/release/rust-academy /app/rust-academy

# Настройки по умолчанию:
# HOST=0.0.0.0 обязателен для работы внутри контейнера
ENV HOST=0.0.0.0 \
    PORT=8011 \
    RA_DB=/app/data/rust-academy.db

EXPOSE 8011

VOLUME ["/app/data"]

CMD ["/app/rust-academy"]
