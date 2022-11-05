pub fn load_env() {
    dotenv::from_filename(".env").ok();
}