use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://doc.rust-lang.org/book/ch05-01-defining-structs.html";
    let result = reqwest::get(url).await?;
    let word: &str = "struct";
    let mut word_count: i32 = 0;
    let mut start: usize = 0;
    if result.status().is_success() {
        let body = result.text().await?;
        while let Some(pos) = body[start..].find(&word) {
            let actual_pos = start + pos;
            word_count += 1;
            start = actual_pos + word.len();
        }
    } else {
        println!("Failed to get html: {}", result.status());
    }
    println!("{} was counted {} time(s).", word, word_count);

    Ok(())
}
