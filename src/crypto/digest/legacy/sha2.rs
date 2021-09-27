use super::{DigestExt, Result};
use ring::digest::{Context, SHA256};

pub struct Sha2<T> {
    inner: T,
}

impl DigestExt<Vec<u8>> for Sha2<Vec<u8>> {
    fn new(bytes: &[u8]) -> Self {
        return Sha2 {
            inner: bytes.to_vec(),
        };
    }
    fn digest(&self) -> Result<Vec<u8>> {
        let mut ctx = Context::new(&SHA256);
        ctx.update(&self.inner);
        Ok(ctx.finish().as_ref().to_vec())
    }
}
