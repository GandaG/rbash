#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use rbash;

use super::collection::Collection;
use super::modfile::ModFile;

#[pyclass(module = "rbash")]
pub struct Record {
    pub(super) raw: rbash::Record,
}

#[pymethods]
impl Record {
    fn long_name(&self, formid: u32, is_mgef: bool) -> &str {
        self.raw.long_name(formid, is_mgef)
    }

    #[getter]
    fn r#mod(&self) -> ModFile {
        ModFile {
            raw: self.raw.r#mod(),
        }
    }

    #[getter]
    fn collection(&self) -> Collection {
        Collection {
            raw: self.raw.collection(),
        }
    }

    fn is_winning(&self, extended_conflicts: bool) -> bool {
        self.raw.is_winning(extended_conflicts)
    }

    #[getter]
    fn is_invalid(&self) -> bool {
        self.raw.is_invalid()
    }

    fn conflict_num(&self, extended_conflicts: bool) -> i32 {
        self.raw.conflict_num(extended_conflicts)
    }

    fn conflicts(&self, extended_conflicts: bool) -> Vec<Record> {
        self.raw
            .conflicts(extended_conflicts)
            .into_iter()
            .map(|raw| Record { raw })
            .collect()
    }

    fn history(&self) -> Vec<Record> {
        self.raw
            .history()
            .into_iter()
            .map(|raw| Record { raw })
            .collect()
    }

    fn copy_into(
        &self,
        dest: &ModFile,
        new_formid: u32,
        new_edid: &str,
        parent: &Record,
        flags: i32,
    ) -> PyResult<Record> {
        let flags = rbash::RecordFlags::from_bits(flags)
            .ok_or_else(|| PyErr::new::<ValueError, _>("Incorrect RecordFlags value."))?;
        let raw = self
            .raw
            .copy_into(&dest.raw, new_formid, new_edid, &parent.raw, flags);
        Ok(Record { raw })
    }

    fn update_references(&self, py: Python, formid_map: PyObject) -> PyResult<Vec<u32>> {
        let dict: &PyDict = formid_map.extract(py)?;
        let mut map: HashMap<u32, u32> = HashMap::with_capacity(dict.len());
        for (key, val) in dict.iter() {
            map.insert(key.extract()?, val.extract()?);
        }
        Ok(self.raw.update_references(&map))
    }

    fn reset(&self) {
        self.raw.reset()
    }

    fn set_id(&self, formid: u32, edid: &str) {
        self.raw.set_id(formid, edid)
    }

    #[args(a = "0", b = "0", c = "0", d = "0", e = "0", f = "0", f = "0")]
    fn field_attribute(
        &self,
        attribute: u32,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
    ) -> u32 {
        let fields = [a, b, c, d, e, f, g];
        self.raw.field_attribute(fields, attribute)
    }

    #[args(a = "0", b = "0", c = "0", d = "0", e = "0", f = "0", f = "0")]
    fn get_field(
        &self,
        byte_len: usize,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
    ) -> Vec<u8> {
        let fields = [a, b, c, d, e, f, g];
        self.raw.get_field(fields, byte_len)
    }

    #[args(a = "0", b = "0", c = "0", d = "0", e = "0", f = "0", f = "0")]
    fn get_field_array(
        &self,
        byte_len: usize,
        length: usize,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
    ) -> Vec<Vec<u8>> {
        let fields = [a, b, c, d, e, f, g];
        self.raw.get_field_array(fields, byte_len, length)
    }

    #[args(a = "0", b = "0", c = "0", d = "0", e = "0", f = "0", f = "0")]
    fn set_field(
        &self,
        length: usize,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
    ) -> Vec<u8> {
        let fields = [a, b, c, d, e, f, g];
        self.raw.set_field(fields, length)
    }

    #[args(a = "0", b = "0", c = "0", d = "0", e = "0", f = "0", f = "0")]
    fn delete_field(&self, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) {
        let fields = [a, b, c, d, e, f, g];
        self.raw.delete_field(fields)
    }

    fn unload(&self) {
        self.raw.unload()
    }

    fn delete(&self) {
        self.raw.delete()
    }
}
