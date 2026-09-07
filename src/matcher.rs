use crate::{
    cli::{Args, Kind},
    glob::Pattern,
};

use anyhow::Result;

use std::{ffi::OsStr, fs::Metadata, os::unix::ffi::OsStrExt};

pub struct Matcher {
    name: Option<Pattern>,
    kind: Option<Kind>,
}

impl Matcher {
    pub fn from_args(args: &Args) -> Result<Matcher> {
        let Some(patt) = &args.name else {
            return Ok(Matcher {
                name: None,
                kind: args.kind,
            });
        };

        let pattern = Pattern::parse(patt.as_bytes());

        Ok(Matcher {
            name: Some(pattern),
            kind: args.kind,
        })
    }

    pub fn matches(&self, name: &OsStr, metadata: &Metadata) -> bool {
        if let Some(pattern) = &self.name {
            if !pattern.matches(name.as_bytes()) {
                return false;
            }
        }

        if let Some(kind) = &self.kind {
            match kind {
                Kind::File => {
                    if !metadata.is_file() {
                        return false;
                    }
                }
                Kind::Dir => {
                    if !metadata.is_dir() {
                        return false;
                    }
                }
                Kind::Symlink => {
                    if !metadata.file_type().is_symlink() {
                        return false;
                    }
                }
            }
        }

        true
    }
}
