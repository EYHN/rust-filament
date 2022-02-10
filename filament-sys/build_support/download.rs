use std::path::{Path, PathBuf};
use std::{env, io, fs};

/// Download a file from the given URL and return the data.
pub fn download(name: impl AsRef<str>, url: impl AsRef<str>) -> io::Result<PathBuf> {
    let resp = ureq::get(url.as_ref()).call();
    match resp {
        Ok(resp) => {
            let mut reader = resp.into_reader();
            let output_path = Path::new(&env::var("OUT_DIR").unwrap())
                .join("download")
                .join(name.as_ref());
            let mut output_file = fs::File::create(&output_path)?;
            io::copy(&mut reader, &mut output_file)?;
            Ok(output_path)
        }
        Err(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
    }
}
