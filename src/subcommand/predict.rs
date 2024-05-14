use std::path::PathBuf;

#[allow(unused_variables)]
pub fn predict_relations(file: PathBuf) -> Result<(), ()> {
    unimplemented!("Argument relations prediction is not yet implemented");
}

#[cfg(test)]
mod tests {
    mod predict_relations {
        use super::super::*;

        // The test by itself isn't really useful but it enables me to test the
        // CI automation in the meanwhile of some real tests.
        #[test]
        #[should_panic]
        fn not_implemented() {
            predict_relations(PathBuf::from("/tmp/")).unwrap();
        }
    }
}