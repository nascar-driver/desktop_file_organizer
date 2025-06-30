use chrono::{DateTime, Datelike, Utc};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

fn get_desktop_path() -> io::Result<PathBuf> {
    dirs::desktop_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Desktop not found"))
}

fn get_archives_path(desktop: &Path) -> PathBuf {
    desktop.join("archives")
}

fn get_yyyymm_from_metadata(path: &Path) -> io::Result<String> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let datetime: DateTime<Utc> = modified_time.into();
    Ok(format!("{:04}{:02}", datetime.year(), datetime.month()))
}

fn get_file_extension(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "unknown".to_string())
}

fn move_file_to_archives(file_path: &Path, archives_root: &Path) -> io::Result<()> {
    let yyyymm = get_yyyymm_from_metadata(file_path)?;
    let file_type = get_file_extension(file_path);

    let target_dir = archives_root.join(&yyyymm).join(&file_type);
    fs::create_dir_all(&target_dir)?;

    let file_name = file_path.file_name().unwrap();
    let target_path = target_dir.join(file_name);

    fs::rename(file_path, target_path)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let desktop = get_desktop_path()?;
    let archives = get_archives_path(&desktop);

    if !archives.exists() {
        fs::create_dir(&archives)?;
    }

    for entry in fs::read_dir(&desktop)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if path.starts_with(&archives) {
                continue;
            }

            match move_file_to_archives(&path, &archives) {
                Ok(_) => println!("Moved: {:?}", path.file_name().unwrap()),
                Err(e) => eprintln!("Failed to move {:?}: {}", path.file_name().unwrap(), e),
            }
        }
    }

    Ok(())
}
