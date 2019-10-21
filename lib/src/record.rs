use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::{c_void, CStr, CString};
use std::ptr::null_mut;
use std::slice;

use bitflags::bitflags;

use super::collection::Collection;
use super::modfile::ModFile;
use super::raw;

bitflags! {
    pub struct RecordFlags: i32 {
        const SET_AS_OVERRIDE = raw::cb_create_flags_t_CB_SET_AS_OVERRIDE;
        const COPY_WINNING_PARENT = raw::cb_create_flags_t_CB_COPY_WINNING_PARENT;
    }
}

pub struct Record {
    pub(super) raw: *mut raw::cb_record_t,
}

impl Record {
    pub fn long_name(&self, formid: u32, is_mgef: bool) -> &str {
        unsafe {
            let c_str = raw::cb_GetLongIDName(self.raw, formid, is_mgef);
            let c_str = CStr::from_ptr(c_str);
            c_str.to_str().unwrap()
        }
    }

    pub fn r#mod(&self) -> ModFile {
        let c_mod = unsafe {
            let c_mod = raw::cb_GetModIDByRecordID(self.raw);
            if c_mod.is_null() {
                panic!("Failed to get record's mod.")
            }
            c_mod
        };
        ModFile { raw: c_mod }
    }

    pub fn collection(&self) -> Collection {
        let c_col = unsafe {
            let c_col = raw::cb_GetCollectionIDByRecordID(self.raw);
            if c_col.is_null() {
                panic!("Failed to get record's collection.")
            }
            c_col
        };
        Collection { raw: c_col }
    }

    pub fn is_winning(&self, extended_conflicts: bool) -> bool {
        let res = unsafe { raw::cb_IsRecordWinning(self.raw, extended_conflicts) };
        if res.is_negative() {
            panic!("Failed to check winning record.")
        }
        res != 0
    }

    pub fn is_invalid(&self) -> bool {
        let res = unsafe { raw::cb_IsRecordFormIDsInvalid(self.raw) };
        if res.is_negative() {
            panic!("Failed to check invalid record.")
        }
        res != 0
    }

    pub fn conflict_num(&self, extended_conflicts: bool) -> i32 {
        unsafe {
            let num = raw::cb_GetNumRecordConflicts(self.raw, extended_conflicts);
            if num.is_negative() {
                panic!("Failed to get record conflicts number.")
            }
            num
        }
    }

    pub fn conflicts(&self, extended_conflicts: bool) -> Vec<Record> {
        let num = self.conflict_num(extended_conflicts);
        let mut recs: Vec<raw::cb_record_t> = Vec::with_capacity(num.try_into().unwrap());
        unsafe {
            if raw::cb_GetRecordConflicts(self.raw, &mut recs.as_mut_ptr(), extended_conflicts)
                .is_negative()
            {
                panic!("Failed to get conflicting records.")
            }
        }
        recs.iter_mut().map(|r| Record { raw: r }).collect()
    }

    pub fn history(&self) -> Vec<Record> {
        let num = self.conflict_num(false); // TODO check correctness
        let mut recs: Vec<raw::cb_record_t> = Vec::with_capacity(num.try_into().unwrap());
        unsafe {
            if raw::cb_GetRecordHistory(self.raw, &mut recs.as_mut_ptr()).is_negative() {
                panic!("Failed to get conflicting records.")
            }
        }
        recs.iter_mut().map(|r| Record { raw: r }).collect()
    }

    pub fn copy_into(
        &self,
        dest: &ModFile,
        new_formid: u32,
        new_edid: &str,
        parent: &Record,
        flags: RecordFlags,
    ) -> Record {
        let c_edid = CString::new(new_edid).unwrap().into_raw();
        let c_flags = flags.bits();
        let c_rec = unsafe {
            let c_rec =
                raw::cb_CopyRecord(self.raw, dest.raw, parent.raw, new_formid, c_edid, c_flags);
            if c_rec.is_null() {
                panic!("Failed to copy record.")
            }
            let _string = CString::from_raw(c_edid);
            c_rec
        };
        Record { raw: c_rec }
    }

    pub fn update_references(&self, formid_map: &HashMap<u32, u32>) -> Vec<u32> {
        super::update_references(None, Some(self), formid_map)
    }

    pub fn reset(&self) {
        unsafe {
            if raw::cb_ResetRecord(self.raw).is_positive() {
                panic!("Failed to reset record.")
            }
        }
    }

    pub fn set_id(&self, formid: u32, edid: &str) {
        let c_edid = CString::new(edid).unwrap().into_raw();
        unsafe {
            raw::cb_SetIDFields(self.raw, formid, c_edid);
            let _string = CString::from_raw(c_edid);
        };
        // TODO No error checking yet since it returns "error" if nothing changes
    }

    pub fn field_attribute(&self, fields: [u32; 7], attribute: u32) -> u32 {
        unsafe {
            raw::cb_GetFieldAttribute(
                self.raw, fields[0], fields[1], fields[2], fields[3], fields[4], fields[5],
                fields[6], attribute,
            )
        }
    }

    pub fn get_field(&self, fields: [u32; 7], byte_len: usize) -> Vec<u8> {
        unsafe {
            let c_vec = raw::cb_GetField(
                self.raw,
                fields[0],
                fields[1],
                fields[2],
                fields[3],
                fields[4],
                fields[5],
                fields[6],
                null_mut::<*mut c_void>(),
            );
            // TODO I really don't like this cast
            let v: &[u8] = slice::from_raw_parts(c_vec as *const u8, byte_len);
            v.to_vec()
        }
    }

    pub fn get_field_array(
        &self,
        fields: [u32; 7],
        byte_len: usize,
        length: usize,
    ) -> Vec<Vec<u8>> {
        // TODO not sure if this actually works
        let mut vals: Vec<Vec<u8>> = Vec::with_capacity(length);
        for _ in 0..length {
            let inner: Vec<u8> = Vec::with_capacity(byte_len);
            vals.push(inner);
        }
        unsafe {
            raw::cb_GetField(
                self.raw,
                fields[0],
                fields[1],
                fields[2],
                fields[3],
                fields[4],
                fields[5],
                fields[6],
                vals.as_mut_ptr() as *mut *mut c_void,
            );
        }
        vals
    }

    pub fn set_field(&self, fields: [u32; 7], length: usize) -> Vec<u8> {
        // TODO this void cast is suspicious
        let mut value: Vec<u8> = Vec::with_capacity(length);
        let c_value = value.as_mut_ptr() as *mut c_void;
        unsafe {
            raw::cb_SetField(
                self.raw,
                fields[0],
                fields[1],
                fields[2],
                fields[3],
                fields[4],
                fields[5],
                fields[6],
                c_value,
                value.len().try_into().unwrap(),
            )
        }
        value
    }

    pub fn delete_field(&self, fields: [u32; 7]) {
        unsafe {
            raw::cb_DeleteField(
                self.raw, fields[0], fields[1], fields[2], fields[3], fields[4], fields[5],
                fields[6],
            )
        }
    }

    pub fn unload(&self) {
        unsafe {
            if raw::cb_UnloadRecord(self.raw).is_positive() {
                panic!("Failed to unload record.")
            }
        }
    }

    // not included in drop since I dunno if it should be auto-deleted
    pub fn delete(&self) {
        unsafe {
            if raw::cb_DeleteRecord(self.raw).is_positive() {
                panic!("Failed to delete record.")
            }
        }
    }
}
