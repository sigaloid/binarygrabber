#![allow(dead_code)]
#![feature(anonymous_lifetime_in_impl_trait)]

use log::{info, trace};
use std::{fs::File, io::Write, path::Path};

pub mod tailwind;
mod tests;
const CHUNK_SIZE: usize = 1024;
pub struct BinaryManifest {
    title: String,
    url: String,
    arm64: String,
    armv7: String,
    x86_64: String,
    x86: String,
    windows: String,
    macos: String,
    linux: String,
}

impl BinaryManifest {
    pub fn new(
        title: String,
        url: String,
        arm64: String,
        armv7: String,
        x86_64: String,
        x86: String,
        windows: String,
        macos: String,
        linux: String,
    ) -> BinaryManifest {
        BinaryManifest {
            title,
            url,
            arm64,
            armv7,
            x86_64,
            x86,
            windows,
            macos,
            linux,
        }
    }
}

pub struct BinaryGrabber {
    manifest: BinaryManifest,
    args: Vec<String>,
}
impl BinaryGrabber {
    pub fn new(manifest: BinaryManifest, args: impl AsRef<[&str]>) -> BinaryGrabber {
        BinaryGrabber {
            manifest,
            args: args
                .as_ref()
                .to_vec()
                .into_iter()
                .map(String::from)
                .collect(),
        }
    }
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Construct path
        let path = Path::new(&self.manifest.title);
        // Check if the file exists
        if !path.exists() {
            trace!("File does not exist - downloading");
            self.grab().unwrap();
        } else {
            trace!("File already exists - using current version");
        }
        let output = std::process::Command::new(format!("./{}", self.manifest.title))
            .args(self.args.clone())
            .output()?;
        println!("{output:?}");

        Ok(())
    }
    pub fn grab(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Download the binary
        #[cfg(target_os = "windows")]
        let os = self.manifest.windows.clone();
        #[cfg(target_os = "macos")]
        let os = self.manifest.macos.clone();
        #[cfg(target_os = "linux")]
        let os = self.manifest.linux.clone();

        #[cfg(target_arch = "aarch64")]
        let arch = self.manifest.arm64.clone();
        #[cfg(target_arch = "arm")]
        let arch = self.manifest.armv7.clone();
        #[cfg(target_arch = "x86_64")]
        let arch = self.manifest.x86_64.clone();
        #[cfg(target_arch = "x86")]
        let arch = self.manifest.x86.clone();

        trace!("OS: {os}, Arch: {arch}");

        let url = self.manifest.url.replace("arch", &arch).replace("os", &os);

        info!("Downloading {} from {url}", self.manifest.title);

        let response = ureq::get(&url).call()?;
        let buf_size = if let Some(val) = response.header("Content-Length") {
            if let Ok(parsed) = val.parse::<usize>() {
                parsed
            } else {
                0
            }
        } else {
            0
        };
        let mut buf = Vec::with_capacity(buf_size);
        response.into_reader().read_to_end(&mut buf)?;

        info!(
            "Downloaded binary, writing {} bytes to {}",
            buf.len(),
            self.manifest.title
        );
        let mut file: File = File::create(&self.manifest.title)?;

        // Set executable bit
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::prelude::PermissionsExt;
            std::fs::set_permissions(&self.manifest.title, std::fs::Permissions::from_mode(0o777))?;
        }

        file.write_all(&buf)?;
        info!("Wrote to {}", self.manifest.title);

        Ok(())
    }
}
