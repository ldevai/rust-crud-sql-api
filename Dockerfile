FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY docker .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN date
RUN cat /etc/localtime && echo America/New_York > /etc/timezone && \
ln -sf /usr/share/zoneinfo/America/New_York /etc/localtime && \
dpkg-reconfigure -f noninteractive tzdata

RUN date && apt-get update && apt-get install -y build-essential libssl-dev clang llvm-dev libclang-dev
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY docker .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin rust-crud-sql-api

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/rust-crud-sql-api /app
CMD ["/app/rust-crud-sql-api"]
