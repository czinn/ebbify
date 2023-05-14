use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use zstd::stream::{read::Decoder, write::Encoder};

use super::{AppData, FileData};
use crate::result::Result;

pub struct SaveFile {
    pub path: PathBuf,
    pub app_data: AppData,
    pub is_sample: bool,
    pub saved_modification: u32,
}

impl SaveFile {
    pub fn new(path: PathBuf) -> Self {
        let app_data = AppData::new();
        let saved_modification = app_data.modification_count();
        Self {
            path,
            app_data,
            is_sample: false,
            saved_modification,
        }
    }

    pub fn load(path: PathBuf) -> Result<Self> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let decompressed_reader = Decoder::new(reader)?;
        let file_data: FileData = serde_json::from_reader(decompressed_reader)?;
        let app_data = AppData::from_file(file_data);
        let saved_modification = app_data.modification_count();
        Ok(Self {
            path,
            app_data,
            is_sample: false,
            saved_modification,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        if !self.is_sample {
            let file = File::create(&self.path)?;
            let writer = BufWriter::new(file);
            let mut compressed_writer = Encoder::new(writer, 10)?;
            serde_json::to_writer(&mut compressed_writer, &self.app_data.file_data())?;
            compressed_writer.finish()?;
            self.saved_modification = self.app_data.modification_count();
        }
        Ok(())
    }

    pub fn load_sample() -> Self {
        let file_data = FileData::sample_data();
        let app_data = AppData::from_file(file_data);
        let saved_modification = app_data.modification_count();
        Self {
            path: Default::default(),
            app_data,
            is_sample: true,
            saved_modification,
        }
    }

    pub fn is_modified(&self) -> bool {
        self.app_data.modification_count() != self.saved_modification
    }
}
