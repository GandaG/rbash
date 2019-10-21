use std::collections::HashMap;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use rbash;
use rbash::prelude::*;

use super::collection::Collection;
use super::record::Record;

#[pyclass(module = "rbash")]
pub struct ModFile {
    pub(super) raw: rbash::ModFile,
}

#[pymethods]
impl ModFile {
    #[getter]
    fn name(&self) -> &str {
        self.raw.name()
    }

    #[getter]
    fn file_name(&self) -> &str {
        self.raw.file_name()
    }

    #[getter]
    fn index(&self) -> i32 {
        self.raw.index()
    }

    fn record_type_num(&self) -> i32 {
        self.raw.record_type_num()
    }

    fn record_types(&self) -> Vec<String> {
        self.raw.record_types()
    }

    fn orphan_record_num(&self) -> i32 {
        self.raw.orphan_record_num()
    }

    fn orphan_records(&self) -> Vec<u32> {
        self.raw.orphan_records()
    }

    fn itm_num(&self) -> i32 {
        self.raw.itm_num()
    }

    fn itms(&self) -> Vec<Record> {
        self.raw
            .itms()
            .into_iter()
            .map(|raw| Record { raw })
            .collect()
    }

    fn record_by_formid(&self, py: Python, id: PyObject) -> PyResult<Record> {
        let id = match id.extract::<u32>(py) {
            Ok(i) => FormID(i),
            Err(_) => EditorID(id.extract::<&str>(py)?),
        };
        let raw = self.raw.record_by_formid(id);
        Ok(Record { raw })
    }

    fn clean_masters(&self) {
        self.raw.clean_masters()
    }

    fn update_references(&self, py: Python, formid_map: PyObject) -> PyResult<Vec<u32>> {
        let dict: &PyDict = formid_map.extract(py)?;
        let mut map: HashMap<u32, u32> = HashMap::with_capacity(dict.len());
        for (key, val) in dict.iter() {
            map.insert(key.extract()?, val.extract()?);
        }
        Ok(self.raw.update_references(&map))
    }

    #[getter]
    fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    fn empty_groups_num(&self) -> i32 {
        self.raw.empty_groups_num()
    }

    fn short_formid(&self, object: u32, is_mgef: bool) -> u32 {
        self.raw.short_formid(object, is_mgef)
    }

    fn create_record(
        &self,
        rec_type: &str,
        rec_formid: u32,
        rec_edid: &str,
        parent: &Record,
        flags: i32,
    ) -> PyResult<Record> {
        let rec_type = convert_rec_type(rec_type);
        let flags = rbash::RecordFlags::from_bits(flags)
            .ok_or_else(|| PyErr::new::<ValueError, _>("Incorrect RecordFlags value."))?;
        let raw = self
            .raw
            .create_record(rec_type, rec_formid, rec_edid, &parent.raw, flags);
        Ok(Record { raw })
    }

    fn record_num(&self, rec_type: &str) -> i32 {
        let rec_type = convert_rec_type(rec_type);
        self.raw.record_num(rec_type)
    }

    fn records(&self, rec_type: &str) -> Vec<Record> {
        let rec_type = convert_rec_type(rec_type);
        self.raw
            .records(rec_type)
            .into_iter()
            .map(|raw| Record { raw })
            .collect()
    }

    fn save(&self, name: &str) {
        self.raw.save(name)
    }

    #[getter]
    fn collection(&self) -> Collection {
        Collection {
            raw: self.raw.collection(),
        }
    }

    fn load(&self) {
        self.raw.load()
    }

    fn unload(&self) {
        self.raw.unload()
    }
}

fn convert_rec_type(rec_type: &str) -> [u8; 4] {
    let array = rec_type.bytes().take(4).collect::<Vec<_>>();
    let mut rec_type = [0_u8; 4];
    rec_type.copy_from_slice(&array[..4]);
    rec_type
}
