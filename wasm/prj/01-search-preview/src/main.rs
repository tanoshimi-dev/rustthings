use search_preview::{build_preview, count_matches};

fn main() {
    let article = "Rust and WebAssembly pair well for responsive interfaces. \
                   A fast search helper can build snippets directly in the browser \
                   without waiting on a backend request.";
    let query = "search";

    println!("Query: {query}");
    println!("Matches: {}", count_matches(article, query));
    println!("Preview: {}", build_preview(article, query, 30));
}
