# Use the official Rust image as the base image
FROM rust:latest

# Install required packages
RUN apt-get update && apt-get install -y curl tar gzip

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


# Create directory for DataDog and copy files
RUN mkdir -p /app/datadog

# Download and extract the DataDog Tracer
RUN TRACER_VERSION=$(curl -s https://api.github.com/repos/DataDog/dd-trace-dotnet/releases/latest | grep tag_name | cut -d '"' -f 4 | cut -c2-) \
    && curl -Lo /tmp/datadog-dotnet-apm-musl.tar.gz https://github.com/DataDog/dd-trace-dotnet/releases/download/v${TRACER_VERSION}/datadog-dotnet-apm-${TRACER_VERSION}-musl.tar.gz \
    && tar -zxvf /tmp/datadog-dotnet-apm-musl.tar.gz -C /app/datadog \
    && rm /tmp/datadog-dotnet-apm-musl.tar.gz

    
# Set the command to run the binary when the container starts
CMD ["./target/release/poc-api-base"]
