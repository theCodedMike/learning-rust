use reqwest::Url;

///
/// cargo r --bin rq
///
fn main() {
    let mut url = Url::parse("https://example.net").expect("Failed to parse");
    println!("{}", url.as_str()); // https://example.net/
    url = url.join("c.png").expect("Failed to join");
    println!("{}\n\n", url.as_str()); // https://example.net/c.png

    let base = Url::parse("https://example.net/a/b.html").expect("Failed to parse");
    let url = base.join("c.png").expect("Failed to join");
    println!("{}\n\n", url.as_str()); // https://example.net/a/c.png

    let base = Url::parse("https://example.net/a/b/").expect("Failed to parse");
    let url = base.join("c.png").expect("Failed to join");
    println!("{}", url.as_str()); // https://example.net/a/b/c.png
}
