FROM rust:1.86

# Install common dependencies
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        curl \
        git \
        vim \
        && rm -rf /var/lib/apt/lists/*

# Create a non-root user (named 'user')
RUN useradd -ms /bin/bash user

# Set up environment variables
ENV PATH="$PATH:/usr/local/go/bin:/home/user/go/bin:/home/user/.cargo/bin"

# Copy project files
COPY . /home/user/rustproject
WORKDIR /home/user/rustproject

# Change ownership to the non-root user
RUN chown -R user:user /home/user/rustproject

# Switch to non-root user
USER user

# Optionally, install common Rust tools (cargo-edit, clippy, rustfmt)
RUN cargo install cargo-edit && \
    rustup component add clippy rustfmt

# Default shell
CMD [ "bash" ]