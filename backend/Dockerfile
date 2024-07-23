FROM rust:1-slim-buster

WORKDIR /usr/src/myapp

# 開発に必要なツールをインストール
RUN apt-get update && apt-get install -y \
    git \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cargo.toml と src ディレクトリを作成
RUN cargo init

# 依存関係をキャッシュするためのダミービルド
COPY Cargo.toml ./
RUN cargo build --release
RUN rm src/*.rs

# プロジェクトのソースコードをコピー
COPY . .

# 開発用コマンド
CMD ["cargo", "run"]

