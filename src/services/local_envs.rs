use dotenv::dotenv;
use std::env;

pub struct Config {
    pub insta_username: String,
    pub insta_password: String,
    pub insta_scraper_path: String,

    pub gpt_token: String,

    pub pv_token: String,

    pub reddit_username: String,
    pub reddit_password: String,
    pub reddit_app_id: String,
    pub reddit_app_secret: String,

    pub mongodb_url: String,
    pub mongodb_database: String,
}

impl Config {
    pub fn from_env(env_file_path: &str) -> Config {
        dotenv().ok(); // Default .env
        dotenv::from_filename(env_file_path).ok(); // Customized .env

        let insta_username = env::var("INSTA_USERNAME").unwrap();
        let insta_password = env::var("INSTA_PASSWORD").unwrap();
        let insta_scraper_path = env::var("INSTA_SCRAPER_PATH").unwrap();
        let gpt_token = env::var("GPT_TOKEN").unwrap();
        let pv_token = env::var("PV_TOKEN").unwrap();
        let reddit_username = env::var("REDDIT_USERNAME").unwrap();
        let reddit_password = env::var("REDDIT_PASSWORD").unwrap();
        let reddit_app_id = env::var("REDDIT_APP_ID").unwrap();
        let reddit_app_secret = env::var("REDDIT_APP_SECRET").unwrap();
        let mongodb_url = env::var("MONGODB_URL").unwrap();
        let mongodb_database = env::var("MONGODB_DATABASE").unwrap();

        Config {
            insta_username,
            insta_password,
            insta_scraper_path,
            gpt_token,
            pv_token,
            reddit_username,
            reddit_password,
            reddit_app_id,
            reddit_app_secret,
            mongodb_url,
            mongodb_database,
        }
    }
}
