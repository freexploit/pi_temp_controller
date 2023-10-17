use tokio::{fs, time::Duration};
use clap::Parser;
use rust_gpiozero::*;
use anyhow::Result;


#[derive(Parser,Debug)]
#[clap(author,version,about, long_about = None)]
struct Args {
    #[clap(short='i', long, default_value_t = 36)]
    min_threshold: i32,
    #[clap(short, long, default_value_t = 45)]
    max_threshold: i32,
    #[clap(short, long, default_value_t = 10)]
    frequency: u64,
    #[clap(short, long, default_value_t = 26)]
    pin_: u8
}


async fn  get_temp () -> Result<i32,  std::io::Error>
{

    let filename = "/sys/class/thermal/thermal_zone0/temp";

    let temp: Result<i32, std::io::Error> = fs::read_to_string(filename).await.and_then(|temp|{
            let no_newline = temp.replace("\n", "");
            let number: i32 = String::from(no_newline).parse().unwrap();
            Ok(number/1000)
    });

    temp
}


#[tokio::main]
async fn main() {
    println!("reading temp");

    let args = Args::parse();

    let pin = args.pin_;
    let max_threshold = args.max_threshold;
    let min_threshold = args.max_threshold;
    let frequency = args.frequency;

    let mut fan = OutputDevice::new(pin);

    loop {

        if let Ok(t) = get_temp().await {
            println!("temp:{} C", t);
            if t >= max_threshold  {
                fan.on();
            } else {
                if t <= max_threshold - min_threshold {
                    fan.off();
                }
            }
        } else {
            println!("weird");
        }
        tokio::time::sleep(Duration::new(frequency, 0)).await;
    }
}
