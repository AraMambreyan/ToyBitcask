mod errors;

pub use errors::{DatabaseError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, LineWriter, Write};
use std::path::PathBuf;

const DB_FILE: &str = "database_file.txt";
const FILE_SIZE_THRESHOLD: u64 = 100_000;
const USEFUL_LOG_ENTRY_THRESHOLD: f64 = 0.5;

#[derive(Debug, Serialize, Deserialize)]
enum LogEntry {
    Set { key: String, value: String },
    Remove { key: String },
}

pub struct KvStore {
    cache: HashMap<String, String>,
    file_path: PathBuf,
    log_entry_count: usize,
}

impl KvStore {
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.cache.get(&key).cloned())
    }

    fn write_line(&mut self, s: String) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        let mut writer = LineWriter::new(BufWriter::new(&mut file));
        writeln!(writer, "{s}")?;
        Ok(())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let s = serde_json::to_string(&LogEntry::Set {
            key: key.clone(),
            value: value.clone(),
        })?;
        match self.write_line(s) {
            Ok(()) => {
                self.cache.insert(key, value);
                self.log_entry_count += 1;
                self.compact_and_read_cache(false)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.get(key.clone())?.is_none() {
            return Err(DatabaseError::KeyNotFound);
        }

        let s = serde_json::to_string(&LogEntry::Remove { key: key.clone() })?;
        match self.write_line(s) {
            Ok(()) => {
                self.cache.remove(&key);
                self.log_entry_count += 1;
                self.compact_and_read_cache(false)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let file_path = path.into().join(DB_FILE);
        let file = File::open(&file_path);
        match file {
            Ok(_) => {}
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&file_path) {
                    Ok(_) => {}
                    Err(e) => return Err(e.into()),
                },
                _ => return Err(error.into()),
            },
        };

        let mut kv_store = KvStore {
            cache: HashMap::new(),
            file_path,
            log_entry_count: 0,
        };

        kv_store.compact_and_read_cache(true)?;
        Ok(kv_store)
    }

    fn should_compact_and_read_cache(&self, force_compaction: bool, file_size: u64) -> bool {
        force_compaction
            || (file_size > FILE_SIZE_THRESHOLD
                && (self.cache.len() as f64 / self.log_entry_count as f64)
                    < USEFUL_LOG_ENTRY_THRESHOLD)
    }

    fn compact_and_read_cache(&mut self, force_compaction: bool) -> Result<()> {
        if !self
            .should_compact_and_read_cache(force_compaction, fs::metadata(&self.file_path)?.len())
        {
            return Ok(());
        }

        let compacted_cache = self.get_compact_cache()?;

        File::create(&self.file_path)?.write_all(b"")?;
        self.cache.clear();

        for (key, value) in compacted_cache {
            self.set(key, value)?;
        }

        Ok(())
    }

    fn get_compact_cache(&self) -> Result<HashMap<String, String>> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut cache = HashMap::new();

        for line in reader.lines() {
            let log_entry = serde_json::from_str(&line.unwrap())?;
            match log_entry {
                LogEntry::Set { key, value } => {
                    cache.insert(key, value);
                }
                LogEntry::Remove { key } => {
                    cache.remove(&key);
                }
            }
        }

        Ok(cache)
    }
}
