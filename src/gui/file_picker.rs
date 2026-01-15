use rfd::AsyncFileDialog;
use std::path::PathBuf;

/// Pick a file using a native dialog
pub async fn pick_file() -> Option<PathBuf> {
    let file = AsyncFileDialog::new()
        .set_title("Select File")
        .pick_file()
        .await;

    file.map(|f| f.path().to_path_buf())
}

/// Pick a folder using a native dialog
pub async fn pick_folder() -> Option<PathBuf> {
    let folder = AsyncFileDialog::new()
        .set_title("Select Folder")
        .pick_folder()
        .await;

    folder.map(|f| f.path().to_path_buf())
}
