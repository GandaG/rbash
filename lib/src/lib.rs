mod collection;
mod modfile;
mod raw;
mod record;

use std::collections::HashMap;
use std::convert::TryInto;
use std::ptr::null_mut;

pub use collection::{Collection, CollectionType};
pub use modfile::{ModFile, ModFlags, RecordOption};
pub use record::{Record, RecordFlags};

pub mod prelude {
    pub use super::RecordOption::*;
}

pub fn cb_version() -> String {
    unsafe {
        format!(
            "{}.{}.{}",
            raw::cb_GetVersionMajor(),
            raw::cb_GetVersionMinor(),
            raw::cb_GetVersionRevision()
        )
    }
}

fn update_references(
    r#mod: Option<&ModFile>,
    rec: Option<&Record>,
    formid_map: &HashMap<u32, u32>,
) -> Vec<u32> {
    let r#mod = match r#mod {
        Some(m) => m.raw,
        None => null_mut(),
    };
    let rec = match rec {
        Some(r) => r.raw,
        None => null_mut(),
    };
    let len = formid_map.len();
    let mut old: Vec<u32> = Vec::with_capacity(len);
    let mut new: Vec<u32> = Vec::with_capacity(len);
    for (key, val) in formid_map.iter() {
        old.push(*key);
        new.push(*val);
    }
    let mut chg: Vec<u32> = Vec::with_capacity(len); // TODO I don't think this is correct
    unsafe {
        if raw::cb_UpdateReferences(
            r#mod,
            rec,
            old.as_mut_ptr(),
            new.as_mut_ptr(),
            chg.as_mut_ptr(),
            len.try_into().unwrap(),
        )
        .is_negative()
        {
            panic!("Failed to update references")
        }
    }
    chg
}
