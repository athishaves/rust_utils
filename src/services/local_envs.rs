use dotenv::dotenv;
use std::env;

pub struct Config {
    pub insta_username: String,
    pub insta_password: String,
    pub insta_scraper_path: String,

    pub gpt_token: String,

    pub pv_token: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();

        let insta_username = env::var("INSTA_USERNAME").unwrap();
        let insta_password = env::var("INSTA_PASSWORD").unwrap();
        let insta_scraper_path = env::var("INSTA_SCRAPER_PATH").unwrap();
        let gpt_token = env::var("GPT_TOKEN").unwrap();
        let pv_token = env::var("PV_TOKEN").unwrap();

        Config {
            insta_username,
            insta_password,
            insta_scraper_path,
            gpt_token,
            pv_token,
        }
    }
}
