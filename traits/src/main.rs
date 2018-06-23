extern crate traits;

use traits::Summriable;

#[derive(Debug)]
struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
}

impl Summriable for WeatherForecast {
    fn summary(&self) -> String {
        format!("the high will be {}, and the low willl be {}", self.high_temp, self.low_temp)
    }
}

fn main() {
    let tweet = traits::Tweet {
        username: String::from("someone"),
        content: String::from("some thing"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let weather = WeatherForecast {high_temp:36.5, low_temp:25.0};
    println!("weather forecast: {}", weather.summary());

    traits::notify(weather);
    traits::notify(tweet);
}
