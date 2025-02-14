use marc_record::{parse_records, ControlField, DataField, Field};

use rustler::{Encoder, Env, Term};
use std::fs::File;
use std::io::Read;

#[rustler::nif]
fn parse_records_wrapper(filename: String) -> Result<Vec<RecordWrapper>, String> {
    let mut contents = Vec::new();
    File::open(filename)
        .unwrap()
        .read_to_end(&mut contents)
        .unwrap();

    let records = parse_records(&contents).unwrap();
    let retval = records
        .iter()
        .map(|record| {
            let fields = record
                .fields
                .iter()
                .map(|field| FieldWrapper::new(field))
                .collect();
            RecordWrapper { fields }
        })
        .collect::<Vec<_>>();
    Ok(retval)
}

struct RecordWrapper {
    pub fields: Vec<FieldWrapper>,
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
            Field::Control(control) => Self::build_control(&control),
            Field::Data(data) => Self::build_data(&data),
        }
    }

    fn build_control(control: &ControlField) -> FieldWrapper {
        FieldWrapper::Control(ControlFieldWrapper {
            tag: control.tag.to_string(),
            data: control.data.clone(),
        })
    }

    fn build_data(data: &DataField) -> FieldWrapper {
        FieldWrapper::Data(DataFieldWrapper {
            tag: data.tag.to_string(),
            indicator: data.indicator.iter().fold(String::new(), |mut acc, &c| {
                acc.push(c);
                acc
            }),
            subfields: data
                .subfields
                .iter()
                .map(|subfield| SubfieldWrapper {
                    tag: subfield.tag.to_string(),
                    data: subfield.data.clone(),
                })
                .collect(),
        })
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
