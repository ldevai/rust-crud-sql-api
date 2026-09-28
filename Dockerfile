# Build
FROM rust:1.98-slim-trixie AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock schema.sql ./
COPY src ./src
RUN cargo build --release --locked

# Run
FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=build /src/target/release/rust-crud-sql-api /usr/local/bin/app
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/app"]
