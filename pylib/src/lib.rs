mod collection;
mod enums;
mod modfile;
mod record;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use rbash as rb;

use collection::Collection;
use enums::*;
use modfile::ModFile;
use record::Record;

#[pymodule]
fn rbash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("CB_VERSION", rb::cb_version())?;
    m.add_wrapped(wrap_pymodule!(CollectionType))?;
    m.add_wrapped(wrap_pymodule!(ModFlags))?;
    m.add_wrapped(wrap_pymodule!(RecordFlags))?;
    m.add_class::<Collection>().unwrap();
    m.add_class::<ModFile>().unwrap();
    m.add_class::<Record>().unwrap();

    Ok(())
}
