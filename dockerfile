# Use a Rust base image with the desired version
FROM rust:1.72.0

# Set a working directory
WORKDIR /app

# Install Git
RUN apt-get update && apt-get install -y git

# Clone the GitHub repository
RUN git clone https://github.com/ShivaBhattacharjee/Synthia.git

# Set the working directory to the cloned repository
WORKDIR /app/Synthia

# Build and run the application using Cargo
RUN cargo build --release
CMD ["cargo", "run"]
