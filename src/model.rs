
pub trait Generate {
    fn generate(&self, question: &str) -> impl std::future::Future<Output = ()> + Send;
}
