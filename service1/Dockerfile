# Use Ubuntu 22.04 as the base image
FROM ubuntu:22.04

RUN apt update -y && apt install -y curl

# Create a directory for your application
WORKDIR /app

# Copy the executable 'service1' and 'Rocket.toml' to the container
COPY target/release/service1 /app/
COPY Rocket.toml /app/

# Expose port 8001
EXPOSE 8001

# Define the command to run when the container starts
CMD ["./service1"]
