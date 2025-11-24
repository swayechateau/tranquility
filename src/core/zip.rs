use std::{
    fs::{self, File},
    io::{self, Read, Seek},
    path::Path,
};
use zip::ZipArchive;

/// Extract a ZIP file safely into a directory
pub fn extract<R: Read + Seek>(
    reader: R,
    dest: &Path,
    only_ttf_otf: bool,
) -> zip::result::ZipResult<()> {
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        // Prevent ZIP slip (../ injection)
        let outpath = match file.enclosed_name() {
            Some(path) => dest.join(path),
            None => continue,
        };

        // Optionally skip non-font files
        if only_ttf_otf {
            if let Some(ext) = outpath.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext != "ttf" && ext != "otf" {
                    continue;
                }
            } else {
                continue;
            }
        }

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
            continue;
        }

        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut outfile = File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;
    }

    Ok(())
}
