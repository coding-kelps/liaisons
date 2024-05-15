use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
struct InputData {
    content: String,
}

#[allow(unused_variables)]
pub fn retrieve_arguments(file_path: PathBuf) -> Result<(), ()> {
    let input: Vec<InputData> = {
        let data = fs::read_to_string(file_path).expect("error reading input file");

        serde_json::from_str(&data).unwrap()
    };

    println!("{:?}", serde_json::to_string_pretty(&input).expect("error parsing input to JSON"));

    Ok(())
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
