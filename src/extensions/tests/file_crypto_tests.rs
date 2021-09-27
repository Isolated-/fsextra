#[cfg(test)]
#[cfg(feature = "crypto")]
mod file_crypto_tests {
    use crate::extensions::file::{DigestAlgorithm, FileExtended};
    use ring::digest::{SHA256_OUTPUT_LEN, SHA384_OUTPUT_LEN, SHA512_OUTPUT_LEN};
    use std::fs::File;

    #[test]
    fn file_digest_returns_sha256() {
        let mut file = File::open("test_data/files/test_01.txt").unwrap();
        let digest = file.digest(DigestAlgorithm::Sha2).unwrap();
        let expected = "7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9";

        assert_eq!(digest.len(), SHA256_OUTPUT_LEN);
        assert_eq!(hex::encode(digest), expected);
    }

    #[test]
    fn file_digest_returns_sha512() {
        let mut file = File::open("test_data/files/test_01.txt").unwrap();
        let digest = file.digest(DigestAlgorithm::Sha5).unwrap();
        let expected = "db9b1cd3262dee37756a09b9064973589847caa8e53d31a9d142ea2701b1b28abd97838bb9a27068ba305dc8d04a45a1fcf079de54d607666996b3cc54f6b67c";

        assert_eq!(digest.len(), SHA512_OUTPUT_LEN);
        assert_eq!(hex::encode(digest), expected);
    }

    #[test]
    fn file_digest_returns_sha384() {
        let mut file = File::open("test_data/files/test_01.txt").unwrap();
        let digest = file.digest(DigestAlgorithm::Sha384).unwrap();
        let expected = "d33d40f7010ce34aa86efd353630309ed5c3d7ffac66d988825cf699f4803ccdf3f033230612f0945332fb580d8af805";

        assert_eq!(digest.len(), SHA384_OUTPUT_LEN);
        assert_eq!(hex::encode(digest), expected);
    }
}
