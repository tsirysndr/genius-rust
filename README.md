![Crates.io](https://img.shields.io/crates/v/genius-rust?color=%2300aa00) ![Crates.io (recent)](https://img.shields.io/crates/dr/genius-rust) ![Crates.io](https://img.shields.io/crates/l/genius-rust)

# genius_rust

Rust library that allows interact with Genius API.

##  Searching for a song

```rust
use genius_rust::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let response = genius.search("Ariana Grande").await.unwrap();
    println!("{}", response[0].result.full_title);
}
```

## Getting lyrics

```rust
use genius_rust::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let response = genius.search("Sia").await.unwrap();
    let lyrics = genius.get_lyrics(&response[0].result.id).await.unwrap();
    for verse in lyrics {
        println!("{}", verse);
    }
}
```

## Getting deeper information for a song by id

```rust
use genius_rust::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let response = genius.search("Weeknd").await.unwrap();
    let song = genius.get_song(response[0].result.id, "plain").await.unwrap();
    println!("{}", song.media.unwrap()[0].url)
}
```
