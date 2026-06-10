fn main() {
    if let Err(e) = ontoindex_lsp::run() {
        eprintln!("ontoindex-lsp error: {e}");
        std::process::exit(1);
    }
}
