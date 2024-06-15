use std::path::PathBuf;
use tokio::fs;

/// Draft of the comming features for argument relations prediction.
#[allow(unused_variables)]
pub async fn predict_relations(file_path: PathBuf) -> Result<(), ()> {
    let _data = fs::read_to_string(file_path).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    mod predict_relations {
    }
}