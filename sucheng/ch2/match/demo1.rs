fn filter(html: &str) {
    for c in html.chars() {
        println!("{}", c);
    }
}

fn filter1(html: &str) {
    match html.len() {
        4 => println!("{}", "ok"),
        _ => println!("{}", "err"),  // default
    }
}

fn filter2(html: &str) {
    match html.len() {
        4..=10 => println!("{}", "ok"),  // [4,10]
        0..=3 => println!("{}", "short"), // [0,3]
        _ => println!("{}", "err"),  // default
    }
}

fn filter3(html: &str) {
    // [1,10]
    for i in 1..=10 {
        println!("{}", i)
    }
    // [1, 3)
    for i in 1..3 {
        println!("{}", i)
    }
}

fn main() {
    let html = "abcd";
    filter(html);
    filter1(html);
    filter2(html);
    let short_html = "ab";
    filter2(short_html);
    filter3(short_html)

}