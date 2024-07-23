use dotenvy::dotenv;

fn main() {
    dotenv().expect("Failed to read .env file");
    // ENV_NAME環境変数を取得し、コンパイル時に設定
    if let Ok(env_name) = std::env::var("ENV_NAME") {
        println!("cargo:rustc-env=ENV_NAME={}", env_name);
    }
}
