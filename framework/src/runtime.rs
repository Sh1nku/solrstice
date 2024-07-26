lazy_static::lazy_static! {
    pub(crate) static ref RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Runtime::new().expect("Failed to create runtime for blocking calls")
    };
}
