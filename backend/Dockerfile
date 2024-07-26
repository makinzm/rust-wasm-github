# ビルドステージ
FROM rust:1-slim-buster as builder

WORKDIR /usr/src/myapp

# 開発に必要なツールをインストール
RUN apt-get update && apt-get install -y \
    git \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# ソースコードをマウントしてビルド
RUN --mount=type=bind,source=.,target=/usr/src/myapp \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/myapp/target \
    cargo build --release && \
    cp target/release/backend /usr/local/bin/myapp

# ビルド成果物の確認（デバッグ用）
RUN ls -l /usr/local/bin/myapp

# 実行ステージ
FROM debian:buster-slim

# 必要なランタイム依存関係をインストール（必要に応じて）
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# ビルドステージから実行可能ファイルをコピー
COPY --from=builder /usr/local/bin/myapp /bin/myapp

CMD ["/bin/myapp"]
