mod constant;
mod kindle;

fn main() {
    println!("{}", kindle::Kindle::scrape_ota());
}
