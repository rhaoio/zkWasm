use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;

use crate::host_function::Signature;
use crate::types::ValueType;

pub mod encode;
mod table;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ExternalHostCallSignature {
    Argument,
    Return,
}

impl ExternalHostCallSignature {
    pub fn is_ret(&self) -> bool {
        *self == ExternalHostCallSignature::Return
    }
}

impl From<ExternalHostCallSignature> for Signature {
    fn from(sig: ExternalHostCallSignature) -> Signature {
        match sig {
            ExternalHostCallSignature::Argument => Signature {
                params: vec![ValueType::I64],
                return_type: None,
            },
            ExternalHostCallSignature::Return => Signature {
                params: vec![],
                return_type: Some(ValueType::I64),
            },
        }
    }
}

pub struct ExternalHostCallEntry {
    pub op: usize,
    pub value: u64,
    pub sig: ExternalHostCallSignature,
}

impl Serialize for ExternalHostCallEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ExternalHostCallEntry", 3)?;
        s.serialize_field("op", &self.op)?;
        s.serialize_field("value", &self.value)?;
        s.serialize_field("is_ret", &self.sig.is_ret())?;
        s.end()
    }
}

#[derive(Default, Serialize)]
pub struct ExternalHostCallTable(pub(crate) Vec<ExternalHostCallEntry>);

impl ExternalHostCallTable {
    pub fn new(entries: Vec<ExternalHostCallEntry>) -> Self {
        Self(entries)
    }

    pub fn entries(&self) -> &Vec<ExternalHostCallEntry> {
        &self.0
    }

    pub fn push(&mut self, entry: ExternalHostCallEntry) {
        self.0.push(entry);
    }
}
