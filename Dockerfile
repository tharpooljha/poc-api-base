# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the working directory
# COPY Cargo.toml Cargo.lock ./

COPY Cargo.toml ./
# Copy the source code to the working directory
COPY src ./src

# Build the release version of the application
RUN cargo build --release

# Expose the port on which the server will run (change if necessary)
EXPOSE 3000

# Set the command to run the binary when the container starts
CMD ["./target/release/poc-api-base"]
