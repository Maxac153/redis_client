# Этап сборки
FROM rust:1.82.0 as builder

# Установка рабочей директории
WORKDIR /usr/src/redis_client

# Копирование файлов конфигурации Cargo
COPY Cargo.toml Cargo.lock ./

# Копирование исходного кода
COPY src ./src

# Копирование папок templates и static
COPY templates ./templates
COPY static ./static

# Добавление целевой платформы для сборки
RUN rustup target add x86_64-unknown-linux-musl

# Установка musl-tools для musl-gcc
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# Запуск тестов в одном потоке
RUN cargo test --target x86_64-unknown-linux-musl -- --test-threads=1

# Сборка приложения в режиме релиза
RUN cargo build --target x86_64-unknown-linux-musl --release

# Этап выполнения
FROM scratch

WORKDIR /home

COPY --from=builder /usr/src/redis_client/target/x86_64-unknown-linux-musl/release/redis_client ./redis_client
COPY --from=builder /usr/src/redis_client/static ./static
COPY --from=builder /usr/src/redis_client/templates ./templates

CMD ["./redis_client"]