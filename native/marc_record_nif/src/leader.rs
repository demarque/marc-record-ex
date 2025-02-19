#![allow(dead_code)]
use rustler::{Encoder, Env, Term};

use marc_record::{
    BibliographicalLevel, CatalogingForm, CodingScheme, ControlType, EncodingLevel, Leader,
    MultipartResourceRecordLevel, RecordType, Status,
};

pub struct LeaderWrapper {
    record_length: u16,
    status: StatusWrapper,
    record_type: RecordTypeWrapper,
    bibliographic_level: BibliographicalLevelWrapper,
    control_type: ControlTypeWrapper,
    coding_scheme: CodingSchemeWrapper,
    data_base_address: u16,
    encoding_level: EncodingLevelWrapper,
    descriptive_cataloging_form: CatalogingFormWrapper,
    multipart_resource_record_level: MultipartResourceRecordLevelWrapper,
}

impl LeaderWrapper {
    pub fn new(leader: Leader) -> Self {
        LeaderWrapper {
            record_length: leader.record_length,
            status: StatusWrapper::new(leader.status),
            record_type: RecordTypeWrapper::new(leader.record_type),
            bibliographic_level: BibliographicalLevelWrapper::new(leader.bibliographical_level),
            control_type: ControlTypeWrapper::new(leader.control_type),
            coding_scheme: CodingSchemeWrapper::new(leader.coding_scheme),
            data_base_address: leader.data_base_address,
            encoding_level: EncodingLevelWrapper::new(leader.encoding_level),
            descriptive_cataloging_form: CatalogingFormWrapper::new(
                leader.descriptive_cataloging_form,
            ),
            multipart_resource_record_level: MultipartResourceRecordLevelWrapper::new(
                leader.multipart_resource_record_level,
            ),
        }
    }
}

impl Encoder for LeaderWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        Term::map_from_pairs(
            env,
            &[
                ("record_length", self.record_length.encode(env)),
                ("status", self.status.encode(env)),
                ("record_type", self.record_type.encode(env)),
                (
                    "bibliographical_level",
                    self.bibliographic_level.encode(env),
                ),
                ("control_type", self.control_type.encode(env)),
                ("coding_scheme", self.coding_scheme.encode(env)),
                ("data_base_address", self.data_base_address.encode(env)),
                ("encoding_level", self.encoding_level.encode(env)),
                (
                    "descriptive_cataloging_form",
                    self.descriptive_cataloging_form.encode(env),
                ),
                (
                    "multipart_resource_record_level",
                    self.multipart_resource_record_level.encode(env),
                ),
            ],
        )
        .expect("Failed to create map: duplicate key")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum StatusWrapper {
    IncreaseInEncoding,
    Corrected,
    Deleted,
    New,
    IncreaseFromPrepublication,
}

impl StatusWrapper {
    pub fn new(status: Status) -> Self {
        match status {
            Status::IncreaseInEncoding => StatusWrapper::IncreaseInEncoding,
            Status::Corrected => StatusWrapper::Corrected,
            Status::Deleted => StatusWrapper::Deleted,
            Status::New => StatusWrapper::New,
            Status::IncreaseFromPrepublication => StatusWrapper::IncreaseFromPrepublication,
        }
    }
}

impl Encoder for StatusWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let status = match self {
            StatusWrapper::IncreaseInEncoding => "increase_in_encoding",
            StatusWrapper::Corrected => "corrected",
            StatusWrapper::Deleted => "deleted",
            StatusWrapper::New => "new",
            StatusWrapper::IncreaseFromPrepublication => "increase_from_prepublication",
        };
        status.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RecordTypeWrapper {
    LanguageMaterial,
    NotatedMusic,
    ManuscriptNotatedMusic,
    CartographicMaterial,
    ManuscriptCartographicMaterial,
    ProjectedMedium,
    NonmusicalSoundRecording,
    MusicalSoundRecording,
    TwoDimensionalNonprojectableGraphic,
    ComputerFile,
    Kit,
    MixedMaterials,
    ThreeDimensionalArtifact,
    ManuscriptLanguageMaterial,
}

impl RecordTypeWrapper {
    pub fn new(record_type: RecordType) -> Self {
        match record_type {
            RecordType::LanguageMaterial => RecordTypeWrapper::LanguageMaterial,
            RecordType::NotatedMusic => RecordTypeWrapper::NotatedMusic,
            RecordType::ManuscriptNotatedMusic => RecordTypeWrapper::ManuscriptNotatedMusic,
            RecordType::CartographicMaterial => RecordTypeWrapper::CartographicMaterial,
            RecordType::ManuscriptCartographicMaterial => {
                RecordTypeWrapper::ManuscriptCartographicMaterial
            }
            RecordType::ProjectedMedium => RecordTypeWrapper::ProjectedMedium,
            RecordType::NonmusicalSoundRecording => RecordTypeWrapper::NonmusicalSoundRecording,
            RecordType::MusicalSoundRecording => RecordTypeWrapper::MusicalSoundRecording,
            RecordType::TwoDimensionalNonprojectableGraphic => {
                RecordTypeWrapper::TwoDimensionalNonprojectableGraphic
            }
            RecordType::ComputerFile => RecordTypeWrapper::ComputerFile,
            RecordType::Kit => RecordTypeWrapper::Kit,
            RecordType::MixedMaterials => RecordTypeWrapper::MixedMaterials,
            RecordType::ThreeDimensionalArtifact => RecordTypeWrapper::ThreeDimensionalArtifact,
            RecordType::ManuscriptLanguageMaterial => RecordTypeWrapper::ManuscriptLanguageMaterial,
        }
    }
}

impl Encoder for RecordTypeWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let record_type = match self {
            RecordTypeWrapper::LanguageMaterial => "language_material",
            RecordTypeWrapper::NotatedMusic => "notated_music",
            RecordTypeWrapper::ManuscriptNotatedMusic => "manuscript_notated_music",
            RecordTypeWrapper::CartographicMaterial => "cartographic_material",
            RecordTypeWrapper::ManuscriptCartographicMaterial => "manuscript_cartographic_material",
            RecordTypeWrapper::ProjectedMedium => "projected_medium",
            RecordTypeWrapper::NonmusicalSoundRecording => "nonmusical_sound_recording",
            RecordTypeWrapper::MusicalSoundRecording => "musical_sound_recording",
            RecordTypeWrapper::TwoDimensionalNonprojectableGraphic => {
                "two_dimensional_nonprojectable_graphic"
            }
            RecordTypeWrapper::ComputerFile => "computer_file",
            RecordTypeWrapper::Kit => "kit",
            RecordTypeWrapper::MixedMaterials => "mixed_materials",
            RecordTypeWrapper::ThreeDimensionalArtifact => "three_dimensional_artifact",
            RecordTypeWrapper::ManuscriptLanguageMaterial => "manuscript_language_material",
        };
        record_type.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BibliographicalLevelWrapper {
    MonographicComponentPart,
    SerialComponentPart,
    Collection,
    Subunit,
    IntegratingResource,
    Monograph,
    Serial,
    Unknown,
}

impl BibliographicalLevelWrapper {
    pub fn new(bibliographic_level: BibliographicalLevel) -> Self {
        match bibliographic_level {
            BibliographicalLevel::MonographicComponentPart => {
                BibliographicalLevelWrapper::MonographicComponentPart
            }
            BibliographicalLevel::SerialComponentPart => {
                BibliographicalLevelWrapper::SerialComponentPart
            }
            BibliographicalLevel::Collection => BibliographicalLevelWrapper::Collection,
            BibliographicalLevel::Subunit => BibliographicalLevelWrapper::Subunit,
            BibliographicalLevel::IntegratingResource => {
                BibliographicalLevelWrapper::IntegratingResource
            }
            BibliographicalLevel::Monograph => BibliographicalLevelWrapper::Monograph,
            BibliographicalLevel::Serial => BibliographicalLevelWrapper::Serial,
            BibliographicalLevel::Unknown => BibliographicalLevelWrapper::Unknown,
        }
    }
}

impl Encoder for BibliographicalLevelWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let bibliographic_level = match self {
            BibliographicalLevelWrapper::MonographicComponentPart => "monographic_component_part",
            BibliographicalLevelWrapper::SerialComponentPart => "serial_component_part",
            BibliographicalLevelWrapper::Collection => "collection",
            BibliographicalLevelWrapper::Subunit => "subunit",
            BibliographicalLevelWrapper::IntegratingResource => "integrating_resource",
            BibliographicalLevelWrapper::Monograph => "monograph",
            BibliographicalLevelWrapper::Serial => "serial",
            BibliographicalLevelWrapper::Unknown => "unknown",
        };
        bibliographic_level.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ControlTypeWrapper {
    Unspecified,
    Archival,
}

impl ControlTypeWrapper {
    pub fn new(control_type: ControlType) -> Self {
        match control_type {
            ControlType::Unspecified => ControlTypeWrapper::Unspecified,
            ControlType::Archival => ControlTypeWrapper::Archival,
        }
    }
}

impl Encoder for ControlTypeWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let control_type = match self {
            ControlTypeWrapper::Unspecified => "unspecified",
            ControlTypeWrapper::Archival => "archival",
        };
        control_type.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CodingSchemeWrapper {
    Marc8,
    Ucs,
}

impl CodingSchemeWrapper {
    pub fn new(coding_scheme: CodingScheme) -> Self {
        match coding_scheme {
            CodingScheme::Marc8 => CodingSchemeWrapper::Marc8,
            CodingScheme::Ucs => CodingSchemeWrapper::Ucs,
        }
    }
}

impl Encoder for CodingSchemeWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let coding_scheme = match self {
            CodingSchemeWrapper::Marc8 => "marc8",
            CodingSchemeWrapper::Ucs => "ucs",
        };
        coding_scheme.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EncodingLevelWrapper {
    Full,
    FullMaterialNotExamined,
    LessThanFullMaterialNotExamined,
    Abbreviated,
    Core,
    Partial,
    Minimal,
    Prepublication,
    Unknown,
    NotApplicable,
    ObsoleteFull,
    ObsoleteMinimal,
    AddedFromBatch,
}

impl EncodingLevelWrapper {
    pub fn new(encoding_level: EncodingLevel) -> Self {
        match encoding_level {
            EncodingLevel::Full => EncodingLevelWrapper::Full,
            EncodingLevel::FullMaterialNotExamined => EncodingLevelWrapper::FullMaterialNotExamined,
            EncodingLevel::LessThanFullMaterialNotExamined => {
                EncodingLevelWrapper::LessThanFullMaterialNotExamined
            }
            EncodingLevel::Abbreviated => EncodingLevelWrapper::Abbreviated,
            EncodingLevel::Core => EncodingLevelWrapper::Core,
            EncodingLevel::Partial => EncodingLevelWrapper::Partial,
            EncodingLevel::Minimal => EncodingLevelWrapper::Minimal,
            EncodingLevel::Prepublication => EncodingLevelWrapper::Prepublication,
            EncodingLevel::Unknown => EncodingLevelWrapper::Unknown,
            EncodingLevel::NotApplicable => EncodingLevelWrapper::NotApplicable,
            EncodingLevel::ObsoleteFull => EncodingLevelWrapper::ObsoleteFull,
            EncodingLevel::ObsoleteMinimal => EncodingLevelWrapper::ObsoleteMinimal,
            EncodingLevel::AddedFromBatch => EncodingLevelWrapper::AddedFromBatch,
        }
    }
}

impl Encoder for EncodingLevelWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let encoding_level = match self {
            EncodingLevelWrapper::Full => "full",
            EncodingLevelWrapper::FullMaterialNotExamined => "full_material_not_examined",
            EncodingLevelWrapper::LessThanFullMaterialNotExamined => {
                "less_than_full_material_not_examined"
            }
            EncodingLevelWrapper::Abbreviated => "abbreviated",
            EncodingLevelWrapper::Core => "core",
            EncodingLevelWrapper::Partial => "partial",
            EncodingLevelWrapper::Minimal => "minimal",
            EncodingLevelWrapper::Prepublication => "prepublication",
            EncodingLevelWrapper::Unknown => "unknown",
            EncodingLevelWrapper::NotApplicable => "not_applicable",
            EncodingLevelWrapper::ObsoleteFull => "obsolete_full",
            EncodingLevelWrapper::ObsoleteMinimal => "obsolete_minimal",
            EncodingLevelWrapper::AddedFromBatch => "added_from_batch",
        };
        encoding_level.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CatalogingFormWrapper {
    NonIsbd,
    Aacr2,
    IsbdPunctuationOmitted,
    IsbdPunctuationIncluded,
    NonIsbdPunctuationOmitted,
    Unknown,
}

impl CatalogingFormWrapper {
    pub fn new(cataloging_form: CatalogingForm) -> Self {
        match cataloging_form {
            CatalogingForm::NonIsbd => CatalogingFormWrapper::NonIsbd,
            CatalogingForm::Aacr2 => CatalogingFormWrapper::Aacr2,
            CatalogingForm::IsbdPunctuationOmitted => CatalogingFormWrapper::IsbdPunctuationOmitted,
            CatalogingForm::IsbdPunctuationIncluded => {
                CatalogingFormWrapper::IsbdPunctuationIncluded
            }
            CatalogingForm::NonIsbdPunctuationOmitted => {
                CatalogingFormWrapper::NonIsbdPunctuationOmitted
            }
            CatalogingForm::Unknown => CatalogingFormWrapper::Unknown,
        }
    }
}

impl Encoder for CatalogingFormWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let cataloging_form = match self {
            CatalogingFormWrapper::NonIsbd => "non_isbd",
            CatalogingFormWrapper::Aacr2 => "aacr2",
            CatalogingFormWrapper::IsbdPunctuationOmitted => "isbd_punctuation_omitted",
            CatalogingFormWrapper::IsbdPunctuationIncluded => "isbd_punctuation_included",
            CatalogingFormWrapper::NonIsbdPunctuationOmitted => "non_isbd_punctuation_omitted",
            CatalogingFormWrapper::Unknown => "unknown",
        };
        cataloging_form.encode(env)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MultipartResourceRecordLevelWrapper {
    NotApplicable,
    Set,
    PartWithIndependentTitle,
    PartWithDependentTitle,
}

impl MultipartResourceRecordLevelWrapper {
    pub fn new(multipart_resource_record_level: MultipartResourceRecordLevel) -> Self {
        match multipart_resource_record_level {
            MultipartResourceRecordLevel::NotApplicable => {
                MultipartResourceRecordLevelWrapper::NotApplicable
            }
            MultipartResourceRecordLevel::Set => MultipartResourceRecordLevelWrapper::Set,
            MultipartResourceRecordLevel::PartWithIndependentTitle => {
                MultipartResourceRecordLevelWrapper::PartWithIndependentTitle
            }
            MultipartResourceRecordLevel::PartwithDependentTitle => {
                MultipartResourceRecordLevelWrapper::PartWithDependentTitle
            }
        }
    }
}

impl Encoder for MultipartResourceRecordLevelWrapper {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let multipart_resource_record_level = match self {
            MultipartResourceRecordLevelWrapper::NotApplicable => "not_applicable",
            MultipartResourceRecordLevelWrapper::Set => "set",
            MultipartResourceRecordLevelWrapper::PartWithIndependentTitle => {
                "part_with_independent_title"
            }
            MultipartResourceRecordLevelWrapper::PartWithDependentTitle => {
                "part_with_dependent_title"
            }
        };
        multipart_resource_record_level.encode(env)
    }
}
