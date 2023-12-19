use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use zip::ZipWriter;

pub struct ZipUtil;

impl ZipUtil {
    pub fn compress_files(files: Vec<&str>, zip_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(zip_file)?;

        let mut zip = ZipWriter::new(file);

        for &file_path in files.iter() {
            let file_content = std::fs::read(file_path)?;
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(std::fs::metadata(file_path)?.permissions().mode());

            zip.start_file(file_path, options)?;
            zip.write_all(&file_content)?;
        }

        zip.finish()?;
        Ok(())
    }
}



