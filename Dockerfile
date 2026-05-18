# ── Stage 1: build ──────────────────────────────────────────────────────────
FROM rust:latest AS builder

WORKDIR /app

# Copy workspace manifest first for layer caching
COPY Cargo.toml ./

# Copy all project Cargo.toml files so cargo can resolve the workspace
COPY my-project/Cargo.toml my-project/
COPY projects/branches/Cargo.toml projects/branches/
COPY projects/enums/Cargo.toml projects/enums/
COPY projects/functions/Cargo.toml projects/functions/
COPY projects/guessing_game/Cargo.toml projects/guessing_game/
COPY projects/hello_world/hello_cargo/Cargo.toml projects/hello_world/hello_cargo/
COPY projects/rectangles/Cargo.toml projects/rectangles/
COPY projects/structs/Cargo.toml projects/structs/
COPY projects/variables/Cargo.toml projects/variables/

# Create dummy main.rs stubs so cargo can build dependencies without full source
RUN for dir in \
        my-project/src \
        projects/branches/src \
        projects/enums/src \
        projects/functions/src \
        projects/guessing_game/src \
        projects/hello_world/hello_cargo/src \
        projects/rectangles/src \
        projects/structs/src \
        projects/variables/src; do \
    mkdir -p "$dir" && echo "fn main() {}" > "$dir/main.rs"; \
    done

# Build dependencies only (cached layer)
RUN cargo build --release 2>/dev/null; true

# Now copy real source and rebuild
# touch forces cargo to detect changes even when COPY doesn't update timestamps
COPY . .
RUN find . -name "*.rs" -not -path "*/target/*" | xargs touch && \
    cargo build --release

# ── Stage 2: runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy all compiled binaries
COPY --from=builder /app/target/release/my-project        ./my-project
COPY --from=builder /app/target/release/branches          ./branches
COPY --from=builder /app/target/release/enums             ./enums
COPY --from=builder /app/target/release/function          ./function
COPY --from=builder /app/target/release/guessing_game     ./guessing_game
COPY --from=builder /app/target/release/hello_cargo       ./hello_cargo
COPY --from=builder /app/target/release/rectangles        ./rectangles
COPY --from=builder /app/target/release/structs           ./structs
COPY --from=builder /app/target/release/variables         ./variables

CMD ["/bin/bash"]
