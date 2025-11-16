FROM postgres:18-bookworm

# pgvector ビルドに必要なパッケージ
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    git \
    postgresql-server-dev-18 \
    && rm -rf /var/lib/apt/lists/*

# pgvector をソースからビルド
WORKDIR /tmp
RUN git clone --branch v0.8.1 https://github.com/pgvector/pgvector.git \
  && cd pgvector \
  && make \
  && make install \
  && cd .. \
  && rm -rf pgvector