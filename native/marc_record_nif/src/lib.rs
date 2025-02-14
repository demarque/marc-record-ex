/// This is the Rust code that will be compiled into a shared library and loaded by Elixir.
/// It is a simple wrapper around the `marc-record` crate that provides a NIF for parsing MARC
/// files.
/// Data Structure in this file are just wrapper for the original data structure from `marc-record` crate which are:
/// - Record: struct with a leader struct and a vector of fields
/// - Field enum for data and control fields
/// - ControlField
/// - DataField
///
/// Each Wrapper implements the Encoder trait from the rustler crate to allow the data to be
/// encoded in a Elixir term which is an Elixir data strcture.
use marc_record::{parse_records, ControlField, DataField, Field, Record, Subfield};

use rustler::{Binary, Encoder, Env, NifResult, Term};

#[rustler::nif]
fn parse_records_wrapper<'a>(data: Binary<'a>) -> NifResult<Vec<RecordWrapper>> {
    let records = parse_records(data.as_slice()).unwrap();
    let retval = records
        .iter()
        .map(|record| RecordWrapper::new(record))
        .collect::<Vec<_>>();
    Ok(retval)
}

struct RecordWrapper {
    pub fields: Vec<FieldWrapper>,
}

impl RecordWrapper {
    pub fn new(record: &Record) -> Self {
        let fields = Self::get_record_fields(record);
        RecordWrapper { fields }
    }

    fn get_record_fields(record: &Record) -> Vec<FieldWrapper> {
        record
            .fields
            .iter()
            .map(|field| FieldWrapper::new(field))
            .collect()
    }
}

impl Encoder for RecordWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let mut data_fields = Term::list_new_empty(env);
        for field in &self.fields {
            data_fields = data_fields.list_prepend(field.encode(env));
        }
        Term::map_from_pairs(env, &[("fields", data_fields)]).unwrap()
    }
}

enum FieldWrapper {
    Data(DataFieldWrapper),
    Control(ControlFieldWrapper),
}

impl FieldWrapper {
    pub fn new(field: &Field) -> Self {
        match field {
            Field::Control(control) => FieldWrapper::Control(ControlFieldWrapper::new(control)),
            Field::Data(data) => FieldWrapper::Data(DataFieldWrapper::new(data)),
        }
    }
}

impl Encoder for FieldWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            FieldWrapper::Data(data) => data.encode(env),
            FieldWrapper::Control(control) => control.encode(env),
        }
    }
}

struct ControlFieldWrapper {
    pub tag: String,
    pub data: String,
}

impl ControlFieldWrapper {
    pub fn new(control: &ControlField) -> Self {
        ControlFieldWrapper {
            tag: control.tag.to_string(),
            data: control.data.clone(),
        }
    }
}

impl Encoder for ControlFieldWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let tag = ("tag", self.tag.encode(env));
        let data = ("data", self.data.encode(env));
        Term::map_from_pairs(env, &[tag, data]).unwrap()
    }
}

struct DataFieldWrapper {
    pub tag: String,
    pub indicator: String,
    pub subfields: Vec<SubfieldWrapper>,
}

impl DataFieldWrapper {
    pub fn new(data: &DataField) -> Self {
        Self {
            tag: data.tag.to_string(),
            indicator: Self::build_indicator(&data.indicator),
            subfields: Self::build_subfields(&data.subfields),
        }
    }

    fn build_indicator(indicator: &[char]) -> String {
        indicator.iter().fold(String::new(), |mut acc, &c| {
            acc.push(c);
            acc
        })
    }

    fn build_subfields(subfields: &[Subfield]) -> Vec<SubfieldWrapper> {
        subfields
            .iter()
            .map(|subfield| SubfieldWrapper {
                tag: subfield.tag.to_string(),
                data: subfield.data.clone(),
            })
            .collect()
    }
}

impl Encoder for DataFieldWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let tag = ("tag", self.tag.encode(env));
        let indicator = ("indicator", self.indicator.encode(env));
        let mut subfields_list = Term::list_new_empty(env);
        for subfield in &self.subfields {
            subfields_list = subfields_list.list_prepend(subfield.encode(env));
        }
        let subfields = ("subfields", subfields_list);
        Term::map_from_pairs(env, &[tag, indicator, subfields]).unwrap()
    }
}

struct SubfieldWrapper {
    pub tag: String,
    pub data: String,
}

impl Encoder for SubfieldWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let mut record = Term::map_new(env);
        record = record.map_put("tag", self.tag.encode(env)).unwrap();
        record.map_put("data", self.data.encode(env)).unwrap()
    }
}

rustler::init!("Elixir.MarcRecordEx");
