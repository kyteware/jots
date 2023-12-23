use std::{path::Path, sync::Arc};

use iced::Application;

use crate::app::Error;

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
        .map_err(|e| Error::Io(e.kind()))
}
