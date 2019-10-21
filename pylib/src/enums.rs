#![allow(non_snake_case)]

use pyo3::prelude::*;

use rbash;

// this is weird since pyo3 does not
// yet support either enums

#[pymodule]
pub fn CollectionType(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add::<i32>("Oblivion", rbash::CollectionType::Oblivion.into())?;
    m.add::<i32>("Fallout3", rbash::CollectionType::Fallout3.into())?;
    m.add::<i32>(
        "FalloutNewVegas",
        rbash::CollectionType::FalloutNewVegas.into(),
    )?;
    m.add::<i32>("Skyrim", rbash::CollectionType::Skyrim.into())?;
    m.add::<i32>("Unknown", rbash::CollectionType::Unknown.into())?;

    Ok(())
}

#[pymodule]
pub fn ModFlags(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add::<i32>("MIN_LOAD", rbash::ModFlags::MIN_LOAD.bits())?;
    m.add::<i32>("FULL_LOAD", rbash::ModFlags::FULL_LOAD.bits())?;
    m.add::<i32>("SKIP_NEW_RECORDS", rbash::ModFlags::SKIP_NEW_RECORDS.bits())?;
    m.add::<i32>("IN_LOAD_ORDER", rbash::ModFlags::IN_LOAD_ORDER.bits())?;
    m.add::<i32>("SAVEABLE", rbash::ModFlags::SAVEABLE.bits())?;
    m.add::<i32>("ADD_MASTERS", rbash::ModFlags::ADD_MASTERS.bits())?;
    m.add::<i32>("LOAD_MASTERS", rbash::ModFlags::LOAD_MASTERS.bits())?;
    m.add::<i32>(
        "EXTENDED_CONFLICTS",
        rbash::ModFlags::EXTENDED_CONFLICTS.bits(),
    )?;
    m.add::<i32>("TRACK_NEW_TYPES", rbash::ModFlags::TRACK_NEW_TYPES.bits())?;
    m.add::<i32>("INDEX_LANDS", rbash::ModFlags::INDEX_LANDS.bits())?;
    m.add::<i32>("FIXUP_PLACEABLES", rbash::ModFlags::FIXUP_PLACEABLES.bits())?;
    m.add::<i32>("CREATE_NEW", rbash::ModFlags::CREATE_NEW.bits())?;
    m.add::<i32>(
        "IGNORE_INACTIVE_MASTERS",
        rbash::ModFlags::IGNORE_INACTIVE_MASTERS.bits(),
    )?;
    m.add::<i32>("SKIP_ALL_RECORDS", rbash::ModFlags::SKIP_ALL_RECORDS.bits())?;

    Ok(())
}

#[pymodule]
pub fn RecordFlags(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add::<i32>(
        "SET_AS_OVERRIDE",
        rbash::RecordFlags::SET_AS_OVERRIDE.bits(),
    )?;
    m.add::<i32>(
        "COPY_WINNING_PARENT",
        rbash::RecordFlags::COPY_WINNING_PARENT.bits(),
    )?;

    Ok(())
}
