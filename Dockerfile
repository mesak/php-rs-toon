FROM php:8.2-cli

# Install dependencies for building Rust extension
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    clang \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Build command (run at runtime to see errors)
CMD ["cargo", "build", "--release"]
