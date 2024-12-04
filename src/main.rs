use rand::Rng;
use std::{
    fmt::format,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

// fn main() -> Result<(), std::io::Error> {
//     println!("Hello, world!");

//     let path = "output/hello.txt";
//     let mut output = File::create(path)?;
//     write!(output, "Rust\nis\nfun")?;

//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);

//     for line in buffered.lines() {
//         println!("{}", line?);
//     }

//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "https://httpbin.org/html";

//     let resp = reqwest::blocking::get(url)?.text()?;
//     println!("{}", resp);
//     Ok(())
// }

const MAX_POKEMON: i32 = 898;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let pokimon_id = rng.gen_range(1..MAX_POKEMON);
    let buse_url = String::from("https://pokeapi.co/api/v2/pokemon/");
    let url = buse_url + &pokimon_id.to_string();
    println!("fetching from {}", url);
    let resp = reqwest::get(url).await?.text().await?; // implement

    // let pokemon = serde_json

    Ok(())
}
