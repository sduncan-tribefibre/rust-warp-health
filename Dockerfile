# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the local Cargo.toml and src/ directory into the container
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build your application
RUN cargo build --release

# Expose the port that your application will run on
EXPOSE 3030

# Set the command to run your application
CMD ["target/release/rust-warp-health"]