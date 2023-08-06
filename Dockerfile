FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /rust-axum-api
# Copy source code 
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

#Create a new stage with a minimal image
FROM scratch
COPY --from=builder /rust-axum-api/target/x86_64-unknown-linux-musl/release/rust-axum-api /rust-axum-api
ENTRYPOINT ["/rust-axum-api"]
EXPOSE 3001