use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::modfile::{ModFile, ModFlags};
use super::raw;
use super::record::Record;

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CollectionType {
    Oblivion = raw::cb_game_type_t_CB_OBLIVION,
    Fallout3 = raw::cb_game_type_t_CB_FALLOUT3,
    FalloutNewVegas = raw::cb_game_type_t_CB_FALLOUT_NEW_VEGAS,
    Skyrim = raw::cb_game_type_t_CB_SKYRIM,
    Unknown = raw::cb_game_type_t_CB_UNKNOWN_GAME_TYPE, // TODO this should not exist - auto-panic
}

pub struct Collection {
    pub(super) raw: *mut raw::cb_collection_t,
}

impl Collection {
    pub fn new(path: &str, kind: CollectionType) -> Collection {
        let c_path = CString::new(path).unwrap().into_raw();
        let c_col = unsafe {
            let c_col = raw::cb_CreateCollection(c_path, kind.into());
            if c_col.is_null() {
                panic!("Failed to create collection.")
            }
            let _string = CString::from_raw(c_path);
            c_col
        };
        Collection { raw: c_col }
    }

    pub fn kind(&self) -> CollectionType {
        let raw = unsafe { raw::cb_GetCollectionType(self.raw) };
        CollectionType::try_from(raw).expect("Failed to parse CollectionType.")
    }

    pub fn add_mod(&self, name: &str, flags: ModFlags) -> ModFile {
        let c_name = CString::new(name).unwrap().into_raw();
        let c_flags = flags.bits();
        let c_mod = unsafe {
            let c_mod = raw::cb_AddMod(self.raw, c_name, c_flags);
            if c_mod.is_null() {
                panic!("Failed to add mod.")
            }
            let _string = CString::from_raw(c_name);
            c_mod
        };
        ModFile { raw: c_mod }
    }

    pub fn mod_num(&self) -> i32 {
        unsafe {
            let mod_num = raw::cb_GetAllNumMods(self.raw);
            if mod_num.is_negative() {
                panic!("Failed to get mod number in collection.")
            }
            mod_num
        }
    }

    pub fn mods(&self) -> Vec<ModFile> {
        let mod_num = self.mod_num();
        let mut mods: Vec<raw::cb_mod_t> = Vec::with_capacity(mod_num.try_into().unwrap());
        unsafe {
            if raw::cb_GetAllModIDs(self.raw, &mut mods.as_mut_ptr()).is_negative() {
                panic!("Failed to get mods in collection.")
            }
        }
        mods.iter_mut().map(|m| ModFile { raw: m }).collect()
    }

    pub fn load_order_num(&self) -> i32 {
        unsafe {
            let mod_num = raw::cb_GetLoadOrderNumMods(self.raw);
            if mod_num.is_negative() {
                panic!("Failed to get load order number.")
            }
            mod_num
        }
    }

    pub fn load_order_mods(&self) -> Vec<ModFile> {
        let mod_num = self.load_order_num();
        let mut mods: Vec<raw::cb_mod_t> = Vec::with_capacity(mod_num.try_into().unwrap());
        unsafe {
            if raw::cb_GetLoadOrderModIDs(self.raw, &mut mods.as_mut_ptr()).is_negative() {
                panic!("Failed to get load order mods.")
            }
        }
        mods.iter_mut().map(|m| ModFile { raw: m }).collect()
    }

    pub fn file_name(&self, index: u32) -> &str {
        unsafe {
            let c_str = raw::cb_GetFileNameByLoadOrder(self.raw, index);
            let c_str = CStr::from_ptr(c_str);
            c_str.to_str().unwrap()
        }
    }

    pub fn mod_name(&self, index: u32) -> &str {
        unsafe {
            let c_str = raw::cb_GetModNameByLoadOrder(self.raw, index);
            let c_str = CStr::from_ptr(c_str);
            c_str.to_str().unwrap()
        }
    }

    pub fn mod_by_name(&self, name: &str) -> ModFile {
        let c_name = CString::new(name).unwrap().into_raw();
        let c_mod = unsafe {
            let c_mod = raw::cb_GetModIDByName(self.raw, c_name);
            if c_mod.is_null() {
                panic!("Failed to get mod by name.")
            }
            let _string = CString::from_raw(c_name);
            c_mod
        };
        ModFile { raw: c_mod }
    }

    pub fn mod_by_index(&self, index: u32) -> ModFile {
        let c_mod = unsafe {
            let c = raw::cb_GetModIDByLoadOrder(self.raw, index);
            if c.is_null() {
                panic!("Failed to get mod by index.")
            }
            c
        };
        ModFile { raw: c_mod }
    }

    pub fn index_by_name(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap().into_raw();
        unsafe {
            let index = raw::cb_GetModLoadOrderByName(self.raw, c_name);
            if index.is_negative() {
                panic!("Failed to get index by name.")
            }
            let _string = CString::from_raw(c_name);
            index
        }
    }

    pub fn has_updated_references(&self, record: Option<&Record>) -> bool {
        let c_rec = match record {
            Some(r) => r.raw,
            None => null_mut(),
        };
        let res = unsafe { raw::cb_GetRecordUpdatedReferences(self.raw, c_rec) };
        if res.is_negative() {
            panic!("Failed to check updated references.")
        }
        res != 0
    }

    pub fn load(&self) {
        // extern "C" fn c_callback(_a: u32, _b: u32, _c: *const ::std::os::raw::c_char) -> bool {
        //     true
        // }
        unsafe {
            if raw::cb_LoadCollection(self.raw, None).is_negative() {
                panic!("Failed to load collection.")
            }
        }
    }

    pub fn unload(&self) {
        unsafe {
            if raw::cb_UnloadCollection(self.raw).is_negative() {
                panic!("Failed to unload collection.")
            }
        }
    }

    pub fn unload_all() {
        unsafe {
            if raw::cb_UnloadAllCollections().is_negative() {
                panic!("Failed to unload all collections.")
            }
        }
    }
}

impl Drop for Collection {
    fn drop(&mut self) {
        unsafe {
            if raw::cb_DeleteCollection(self.raw).is_negative() {
                panic!("Failed to delete collection.")
            }
        }
    }
}
