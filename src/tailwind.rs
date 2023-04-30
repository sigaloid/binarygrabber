use crate::BinaryManifest;

impl BinaryManifest {
    pub fn tailwind() -> Self {
        Self {
            title: "tailwind".to_string(),
            url: "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-os-arch".to_string(),
            arm64: "arm64".to_string(),
            armv7: "armv7".to_string(),
            x86_64: "x64".to_string(),
            x86: "undefined".to_string(), // We just have to fail at runtime for this :(
            windows: "windows".to_string(),
            macos: "macos".to_string(),
            linux: "linux".to_string(),
        }
    }
}
