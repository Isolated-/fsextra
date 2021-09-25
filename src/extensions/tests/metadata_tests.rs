#[cfg(test)]
mod metadata_tests {
    use crate::extensions::metadata::MetadataExtended;
    use std::fs::File;

    #[test]
    fn metadata_is_executable_returns_false() {
        let file = File::open("test_data/test_01.txt").unwrap();
        let metadata = file.metadata().unwrap();

        assert_eq!(metadata.is_executable(), false);
    }

    #[test]
    fn metadata_is_executable_returns_true() {
        let file = File::open("test_data/executable").unwrap();
        let metadata = file.metadata().unwrap();

        assert_eq!(metadata.is_executable(), true);
    }

    #[test]
    fn metadata_is_executable_returns_false_on_directory() {
        let file = File::open("test_data").unwrap();
        let metadata = file.metadata().unwrap();

        assert_eq!(metadata.is_executable(), false);
    }

    #[test]
    fn metadata_is_executable_does_not_change_file() {
        let file = File::open("test_data/test_01.txt").unwrap();
        let metadata = file.metadata().unwrap();

        assert_eq!(metadata.is_executable(), false);

        let comparison = File::open("test_data/test_01.txt")
            .unwrap()
            .metadata()
            .unwrap();

        assert_eq!(metadata.len(), comparison.len());
        assert_eq!(metadata.modified().unwrap(), comparison.modified().unwrap());
        assert_eq!(metadata.accessed().unwrap(), comparison.accessed().unwrap());
    }
}
