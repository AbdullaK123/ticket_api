# The base image
FROM rust:1.88

# set the working dir
WORKDIR /app

# copy app code
COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true

# install dependencies
RUN cargo build --release

# expose port 3000
EXPOSE 3000

# run the app
CMD ["./target/release/outro_08"]

