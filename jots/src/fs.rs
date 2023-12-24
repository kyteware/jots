use std::{path::Path, sync::Arc};

use dirs::data_dir;

use crate::error::Error;

pub async fn prep_data_dir() -> Result<(), Error> {
    let jots_dir = data_dir().ok_or(Error::NoDataDir)?.join("jots");
    for subdir in ["notes"] {
        tokio::fs::create_dir_all(jots_dir.join(subdir))
            .await
            .map_err(|e| Error::Fs(e.kind()))?;
    }
    tokio::fs::create_dir_all(&jots_dir)
        .await
        .map_err(|e| Error::Fs(e.kind()))?;
    Ok(())
}

pub async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Pick a text file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path()).await
}

pub async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|e| Error::Fs(e.kind()))
}
