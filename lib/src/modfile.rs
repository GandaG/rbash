use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;
use std::str::from_utf8;

use bitflags::bitflags;

use super::collection::Collection;
use super::raw;
use super::record::{Record, RecordFlags};

bitflags! {
    pub struct ModFlags: i32 {
        const MIN_LOAD = raw::cb_mod_flags_t_CB_MIN_LOAD;
        const FULL_LOAD = raw::cb_mod_flags_t_CB_FULL_LOAD;
        const SKIP_NEW_RECORDS = raw::cb_mod_flags_t_CB_SKIP_NEW_RECORDS;
        const IN_LOAD_ORDER = raw::cb_mod_flags_t_CB_IN_LOAD_ORDER;
        const SAVEABLE = raw::cb_mod_flags_t_CB_SAVEABLE;
        const ADD_MASTERS = raw::cb_mod_flags_t_CB_ADD_MASTERS;
        const LOAD_MASTERS = raw::cb_mod_flags_t_CB_LOAD_MASTERS;
        const EXTENDED_CONFLICTS = raw::cb_mod_flags_t_CB_EXTENDED_CONFLICTS;
        const TRACK_NEW_TYPES = raw::cb_mod_flags_t_CB_TRACK_NEW_TYPES;
        const INDEX_LANDS = raw::cb_mod_flags_t_CB_INDEX_LANDS;
        const FIXUP_PLACEABLES = raw::cb_mod_flags_t_CB_FIXUP_PLACEABLES;
        const CREATE_NEW = raw::cb_mod_flags_t_CB_CREATE_NEW;
        const IGNORE_INACTIVE_MASTERS = raw::cb_mod_flags_t_CB_IGNORE_INACTIVE_MASTERS;
        const SKIP_ALL_RECORDS = raw::cb_mod_flags_t_CB_SKIP_ALL_RECORDS;
    }
}

pub enum RecordOption<'a> {
    FormID(u32),
    EditorID(&'a str),
}

pub struct ModFile {
    pub(super) raw: *mut raw::cb_mod_t,
}

impl ModFile {
    pub fn name(&self) -> &str {
        unsafe {
            let c_str = raw::cb_GetModNameByID(self.raw);
            let c_str = CStr::from_ptr(c_str);
            c_str.to_str().unwrap()
        }
    }

    pub fn file_name(&self) -> &str {
        unsafe {
            let c_str = raw::cb_GetFileNameByID(self.raw);
            let c_str = CStr::from_ptr(c_str);
            c_str.to_str().unwrap()
        }
    }

    pub fn index(&self) -> i32 {
        unsafe {
            let index = raw::cb_GetModLoadOrderByID(self.raw);
            if index.is_negative() {
                panic!("Failed to get mod index.")
            }
            index
        }
    }

    pub fn record_type_num(&self) -> i32 {
        unsafe {
            let num = raw::cb_GetModNumTypes(self.raw);
            if num.is_negative() {
                panic!("Failed to get number of record types.")
            }
            num
        }
    }

    pub fn record_types(&self) -> Vec<String> {
        let num = self.record_type_num();
        let mut recs: Vec<u32> = Vec::with_capacity(num.try_into().unwrap());
        unsafe {
            if raw::cb_GetModTypes(self.raw, recs.as_mut_ptr()).is_negative() {
                panic!("Failed to get record types in mod.")
            }
        }
        recs.iter_mut()
            .map(|i| from_utf8(&i.to_be_bytes()).unwrap().to_string()) // TODO ensure endianness
            .collect()
    }

    pub fn orphan_record_num(&self) -> i32 {
        unsafe {
            let num = raw::cb_GetModNumOrphans(self.raw);
            if num.is_negative() {
                panic!("Failed to get number of orphan records.")
            }
            num
        }
    }

    pub fn orphan_records(&self) -> Vec<u32> {
        let num = self.orphan_record_num();
        let mut recs: Vec<u32> = Vec::with_capacity(num.try_into().unwrap());
        unsafe {
            if raw::cb_GetModOrphansFormIDs(self.raw, recs.as_mut_ptr()).is_negative() {
                panic!("Failed to get orphan records.")
            }
        }
        recs
    }

    pub fn itm_num(&self) -> i32 {
        unsafe {
            let num = raw::cb_GetNumIdenticalToMasterRecords(self.raw);
            if num.is_negative() {
                panic!("Failed to get number of ITM records.")
            }
            num
        }
    }

    pub fn itms(&self) -> Vec<Record> {
        let num = self.itm_num();
        let mut recs: Vec<raw::cb_record_t> = Vec::with_capacity(num.try_into().unwrap());
        unsafe {
            if raw::cb_GetIdenticalToMasterRecords(self.raw, &mut recs.as_mut_ptr()).is_negative() {
                panic!("Failed to get ITM records.")
            }
        }
        recs.iter_mut().map(|r| Record { raw: r }).collect()
    }

    pub fn record_by_formid(&self, id: RecordOption) -> Record {
        use RecordOption::*;

        let c_rec = match id {
            FormID(f) => unsafe { raw::cb_GetRecordID(self.raw, f, null_mut()) },
            EditorID(e) => unsafe {
                let c_edid = CString::new(e).unwrap().into_raw();
                let c_rec = raw::cb_GetRecordID(self.raw, 0, c_edid);
                let _string = CString::from_raw(c_edid);
                c_rec
            },
        };
        if c_rec.is_null() {
            panic!("Failed to get record by formid.")
        }
        Record { raw: c_rec }
    }

    pub fn clean_masters(&self) {
        unsafe {
            if raw::cb_CleanModMasters(self.raw).is_negative() {
                panic!("Failed to clean masters.")
            }
        }
    }

    pub fn update_references(&self, formid_map: &HashMap<u32, u32>) -> Vec<u32> {
        super::update_references(Some(self), None, formid_map)
    }

    pub fn is_empty(&self) -> bool {
        unsafe { raw::cb_IsModEmpty(self.raw) != 0 }
    }

    pub fn empty_groups_num(&self) -> i32 {
        unsafe {
            let num = raw::cb_GetModNumEmptyGRUPs(self.raw);
            if num.is_negative() {
                panic!("Failed to get empty record groups number")
            }
            num
        }
    }

    pub fn short_formid(&self, object: u32, is_mgef: bool) -> u32 {
        unsafe {
            let num = raw::cb_MakeShortFormID(self.raw, object, is_mgef);
            if num == 0 {
                panic!("Failed to make short formID.")
            }
            num
        }
    }

    pub fn create_record(
        &self,
        rec_type: [u8; 4],
        rec_formid: u32,
        rec_edid: &str,
        parent: &Record,
        flags: RecordFlags,
    ) -> Record {
        let rec_type = u32::from_be_bytes(rec_type); // TODO check endianness
        let c_edid = CString::new(rec_edid).unwrap().into_raw();
        let c_flags = flags.bits();
        let c_rec = unsafe {
            let c_rec =
                raw::cb_CreateRecord(self.raw, rec_type, rec_formid, c_edid, parent.raw, c_flags);
            if c_rec.is_null() {
                panic!("Failed to create record.")
            }
            let _string = CString::from_raw(c_edid);
            c_rec
        };
        Record { raw: c_rec }
    }

    pub fn record_num(&self, rec_type: [u8; 4]) -> i32 {
        let rec_type = u32::from_be_bytes(rec_type); // TODO check endianness
        let num = unsafe { raw::cb_GetNumRecords(self.raw, rec_type) };
        if num.is_negative() {
            panic!("Failed to get number of records of type ???")
        }
        num
    }

    pub fn records(&self, rec_type: [u8; 4]) -> Vec<Record> {
        let num = self.record_num(rec_type);
        let rec_type = u32::from_be_bytes(rec_type); // TODO check endianness
        let mut recs: Vec<raw::cb_record_t> = Vec::with_capacity(num.try_into().unwrap());
        unsafe {
            if raw::cb_GetRecordIDs(self.raw, rec_type, &mut recs.as_mut_ptr()).is_negative() {
                panic!("Failed to get records of type ???")
            }
        }
        recs.iter_mut().map(|r| Record { raw: r }).collect()
    }

    pub fn save(&self, name: &str) {
        let c_name = CString::new(name).unwrap().into_raw();
        unsafe {
            if raw::cb_SaveMod(self.raw, 0, c_name).is_negative() {
                panic!("Failed to save mod.")
            }
            let _string = CString::from_raw(c_name);
        }
    }

    pub fn collection(&self) -> Collection {
        let c_col = unsafe {
            let c_col = raw::cb_GetCollectionIDByModID(self.raw);
            if c_col.is_null() {
                panic!("Failed to get mod's collection.")
            }
            c_col
        };
        Collection { raw: c_col }
    }

    pub fn load(&self) {
        unsafe {
            if raw::cb_LoadMod(self.raw).is_negative() {
                panic!("Failed to load mod.")
            }
        }
    }

    pub fn unload(&self) {
        unsafe {
            if raw::cb_UnloadMod(self.raw).is_negative() {
                panic!("Failed to unload mod.")
            }
        }
    }
}
