//! Generic Watcher implementation based on polling
//!
//! Checks the `watch`ed paths periodically to detect changes. This implementation only uses
//! Rust stdlib APIs and should work on all of the platforms it supports.

use self::walkdir::WalkDir;
use super::debounce::{Debounce, EventTx};
use super::{op, DebouncedEvent, Error, RawEvent, RecursiveMode, Result, Watcher};
use filetime::FileTime;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

extern crate walkdir;

struct PathData {
    mtime: i64,
    last_check: Instant,
}

struct WatchData {
    is_recursive: bool,
    paths: HashMap<PathBuf, PathData>,
}

