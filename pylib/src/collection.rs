use std::convert::TryFrom;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;

use rbash;

use super::modfile::ModFile;
use super::record::Record;

#[pyclass(module = "rbash")]
pub struct Collection {
    pub(super) raw: rbash::Collection,
}

#[pymethods]
impl Collection {
    #[new]
    fn init(obj: &PyRawObject, path: &str, kind: i32) -> PyResult<()> {
        let kind = rbash::CollectionType::try_from(kind)
            .map_err(|_| PyErr::new::<ValueError, _>("Incorrect CollectionType value."))?;
        let raw = rbash::Collection::new(path, kind);
        obj.init({ Collection { raw } });
        Ok(())
    }

    #[getter]
    fn kind(&self) -> i32 {
        self.raw.kind().into()
    }

    fn add_mod(&self, name: &str, flags: i32) -> PyResult<ModFile> {
        let flags = rbash::ModFlags::from_bits(flags)
            .ok_or_else(|| PyErr::new::<ValueError, _>("Incorrect ModFlags value."))?;
        let raw = self.raw.add_mod(name, flags);
        Ok(ModFile { raw })
    }

    fn mod_num(&self) -> i32 {
        self.raw.mod_num()
    }

    fn mods(&self) -> Vec<ModFile> {
        self.raw
            .mods()
            .into_iter()
            .map(|raw| ModFile { raw })
            .collect()
    }

    fn load_order_num(&self) -> i32 {
        self.raw.load_order_num()
    }

    fn load_order_mods(&self) -> Vec<ModFile> {
        self.raw
            .load_order_mods()
            .into_iter()
            .map(|raw| ModFile { raw })
            .collect()
    }

    fn file_name(&self, index: u32) -> &str {
        self.raw.file_name(index) // TODO index>255?
    }

    fn mod_name(&self, index: u32) -> &str {
        self.raw.mod_name(index) // TODO missing mod name?
    }

    fn mod_by_name(&self, name: &str) -> ModFile {
        ModFile {
            raw: self.raw.mod_by_name(name),
        } // TODO missing mod name?
    }

    fn mod_by_index(&self, index: u32) -> ModFile {
        ModFile {
            raw: self.raw.mod_by_index(index),
        } // TODO index>255?
    }

    fn index_by_name(&self, name: &str) -> i32 {
        self.raw.index_by_name(name) // TODO missing mod
    }

    fn has_updated_references(&self, record: Option<&Record>) -> bool {
        self.raw.has_updated_references(record.map(|r| &r.raw))
    }

    fn load(&self) {
        self.raw.load()
    }

    fn unload(&self) {
        self.raw.unload()
    }

    #[staticmethod]
    fn unload_all() {
        rbash::Collection::unload_all()
    }
}
