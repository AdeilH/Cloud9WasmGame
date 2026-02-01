# Build stage
FROM rust:1.92-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk
RUN cargo install --locked trunk

WORKDIR /app
COPY . .

# Build the project
RUN trunk build --release

# Serve stage
FROM nginx:alpine

# Copy the built files
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy the nginx config template
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Use a command that replaces the $PORT variable in nginx.conf
# Railway provides the PORT environment variable. Nginx defaults to 80 if PORT is not set.
CMD sed -i -e 's/$PORT/'"${PORT:-80}"'/g' /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'
