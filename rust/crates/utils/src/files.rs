/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Check or create dir.
pub fn check_or_create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

/// Write file.
pub fn write_file<P: AsRef<Path>, T: AsRef<[u8]>>(path: P, data: T) -> std::io::Result<()> {
    fs::write(path, data)
}

/// Read file.
pub fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}

/// Remove file.
pub fn remove_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::remove_file(path)
}

/// ApiAssets.
/// The assets api.
#[derive(Clone)]
pub struct ApiAssets {
    image: PathBuf,
    audio: PathBuf,
    other: PathBuf,
}

impl ApiAssets {
    /// Create new ApiAssets.
    pub fn new(root: &str) -> std::io::Result<Self> {
        let root = PathBuf::from(root);
        let image = root.join("images");
        let audio = root.join("audio");
        let other = root.join("other");
        check_or_create_dir(&image)?;
        check_or_create_dir(&audio)?;
        check_or_create_dir(&other)?;
        Ok(Self {
            image,
            audio,
            other,
        })
    }

    /// Write to image.
    pub fn write_image<T: AsRef<[u8]>>(&self, name: &str, data: T) -> std::io::Result<()> {
        write_file(self.image.join(name), data)
    }

    /// Check if image exists.
    pub fn image_exists(&self, name: &str) -> bool {
        self.image.join(name).exists()
    }

    /// Read image.
    pub fn read_image(&self, name: &str) -> std::io::Result<Vec<u8>> {
        read_file(self.image.join(name))
    }

    /// Remove image.
    pub fn remove_image(&self, name: &str) -> std::io::Result<()> {
        remove_file(self.image.join(name))
    }

    /// Write to audio.
    pub fn write_audio<T: AsRef<[u8]>>(&self, name: &str, data: T) -> std::io::Result<()> {
        write_file(self.audio.join(name), data)
    }

    /// Check if audio exists.
    pub fn audio_exists(&self, name: &str) -> bool {
        self.audio.join(name).exists()
    }

    /// Read audio.
    pub fn read_audio(&self, name: &str) -> std::io::Result<Vec<u8>> {
        read_file(self.audio.join(name))
    }

    /// Get audio path.
    pub fn audio_path(&self, name: &str) -> String {
        self.audio.join(name).to_str().unwrap().to_string()
    }

    /// Remove audio.
    pub fn remove_audio(&self, name: &str) -> std::io::Result<()> {
        remove_file(self.audio.join(name))
    }

    #[cfg(feature = "api")]
    /// Calculate audio duration.
    pub fn duration_audio(&self, name: &str) -> Result<u64, String> {
        super::audio::duration_of(self.audio.join(name).to_str().unwrap())
    }

    /// Write to other.
    pub fn write_other<T: AsRef<[u8]>>(&self, name: &str, data: T) -> std::io::Result<()> {
        write_file(self.other.join(name), data)
    }

    /// Check if other exists.
    pub fn other_exists(&self, name: &str) -> bool {
        self.other.join(name).exists()
    }

    /// Read other.
    pub fn read_other(&self, name: &str) -> std::io::Result<Vec<u8>> {
        read_file(self.other.join(name))
    }

    /// Remove other.
    pub fn remove_other(&self, name: &str) -> std::io::Result<()> {
        remove_file(self.other.join(name))
    }
}
