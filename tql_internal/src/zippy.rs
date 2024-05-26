//!
//! omg it's zippy frfr
//!

use anyhow::Result;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::{fs, io};

pub fn extract_zip<R>(zip: &mut R, dest_folder: &Path) -> Result<()>
where
    for<'a> &'a mut R: Read + Seek,
{
    let mut archive = zip::ZipArchive::new(zip)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => {
                println!("sussy file {} skipped", i);
                continue;
            }
        };

        if file.is_dir() {
            println!("folder {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "file {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // get and set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                    .unwrap();
            }
        }
    }

    Ok(())
}
