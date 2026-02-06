#[derive(Debug, Clone)]
pub struct Chunk {
    pub heading_path: Option<String>,
    pub text: String,
    pub byte_offset: usize,
    pub byte_length: usize,
    pub token_count: usize,
}
