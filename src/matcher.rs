use crate::{
    cli::{Args, Kind},
    glob::Pattern,
};
use std::{ffi::OsStr, fs::Metadata, os::unix::ffi::OsStrExt};

pub struct Matcher {
    name: Option<Pattern>,
    kind: Option<Kind>,
}

impl Matcher {
    pub fn from_args(args: &Args) -> anyhow::Result<Matcher> {
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

    if let Some(name) = &args.name {
        let Some(filename) = path.file_name() else {
                return false;
        };

        if filename != name.as_str() {
            return false;
        }
    }

    true
}
