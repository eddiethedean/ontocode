fn main() {
    if let Err(e) = ontocore_lsp::run() {
        eprintln!("ontocore-lsp error: {e}");
        std::process::exit(1);
    }
}
