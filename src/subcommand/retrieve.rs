use std::path::PathBuf;

#[allow(unused_variables)]
pub fn retrieve_arguments(file: PathBuf) -> Result<(), ()> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    mod retrieve_arguments {
        use super::super::*;

        // The test by itself isn't really useful but it enables me to test the
        // CI automation in the meanwhile of some real tests.
        #[test]
        #[should_panic]
        fn not_implemented() {
            retrieve_arguments(PathBuf::from("/tmp/")).unwrap();
        }
    }
}
