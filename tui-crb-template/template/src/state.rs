use ratatui::Frame;
use reqwest::Client;
use reqwest::Url;
use serde_json::{from_str, Value};
use std::time::SystemTime;

pub struct AppState {
    crabs: usize,
	blockheight: String,
}

impl AppState {
    pub fn new() -> Self {
        Self { crabs: 0, blockheight: 0.to_string() }
    }

    pub fn get_blockheight(&mut self) -> &String {
        let since_the_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("get millis error");
        let seconds = since_the_epoch.as_secs();
        let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
        let blockheight = reqwest::blocking::get(url).unwrap().text().unwrap();
        let blockheight_i32 = blockheight.parse::<i32>().unwrap_or(0);
        let weeble = seconds as f64 / blockheight_i32 as f64;
        let wobble = seconds as f64 % blockheight_i32 as f64;
        self.blockheight = format!("{}/{:}/{}", weeble.floor(), blockheight_i32, wobble);
        &self.blockheight
    }

    pub fn plus_zero(&mut self) {
        self.crabs = 0;
    }
    pub fn plus_one(&mut self) {
        self.crabs += 1;
    }
    pub fn minus_one(&mut self) {
		if self.crabs >= 1 {
        self.crabs -= 1;
		}
	}

    pub fn render(&self, frame: &mut Frame<'_>) {
        let mut welcome_text = String::from("Hello, CRaBs! ");
        let blockheight = String::from(&self.blockheight);
        for _ in 0..self.crabs {
            welcome_text.push('🦀');
        }
        let text_string = format!("{} {}", blockheight, welcome_text);
        frame.render_widget(text_string, frame.area());
    }
}
