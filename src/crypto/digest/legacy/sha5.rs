use super::{DigestExt, Result};
use ring::digest::{Context, SHA512};

pub struct Sha5<T> {
    inner: T,
}

impl DigestExt<Vec<u8>> for Sha5<Vec<u8>> {
    fn new(bytes: &[u8]) -> Self {
        return Sha5 {
            inner: bytes.to_vec(),
        };
    }

    fn digest(&self) -> Result<Vec<u8>> {
        let mut ctx = Context::new(&SHA512);
        ctx.update(&self.inner);
        Ok(ctx.finish().as_ref().to_vec())
    }
}
