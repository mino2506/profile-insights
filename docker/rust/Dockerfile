# dev.Dockerfile
FROM rust:1-slim-bookworm

WORKDIR /app

# ホットリロード用ツール
RUN cargo install cargo-watch

# OpenCVのインストール
RUN apt-get update \
  && apt-get install -y --no-install-recommends libopencv-dev \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 3000
CMD ["cargo", "watch", "-x", "run"]
