fn main() {
    let start = tokio::time::Instant::now();
    
    let body = async {
        
    };
    
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build Tokio Runtime.")
            .block_on(body);
    }
}
