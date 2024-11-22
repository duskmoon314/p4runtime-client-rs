//! P4Info table helper struct and methods.

use std::collections::HashMap;
use std::ops::Deref;

use p4runtime::p4::config::v1 as p4_cfg_v1;
use p4runtime::p4::v1 as p4_v1;

use crate::error::{MakeFieldMatchError, MakeTableActionError};

/// Field match value.
///
/// This enum is used to represent the second value of a field match.
pub enum FieldMatchValue {
    /// A byte vector.
    Vec(Vec<u8>),

    /// A 32-bit integer.
    I32(i32),
}

impl From<Vec<u8>> for FieldMatchValue {
    fn from(v: Vec<u8>) -> Self {
        FieldMatchValue::Vec(v)
    }
}

impl From<i32> for FieldMatchValue {
    fn from(v: i32) -> Self {
        FieldMatchValue::I32(v)
    }
}

/// P4Info table helper struct.
///
/// Wraps around a `p4_cfg_v1::Table` message and provides helper methods.
#[derive(Clone, Debug)]
pub struct Table<'a> {
    table: &'a p4_cfg_v1::Table,

    match_field_map: HashMap<&'a str, p4_cfg_v1::MatchField>,
    action_map: HashMap<&'a str, p4_cfg_v1::Action>,
}

impl Deref for Table<'_> {
    type Target = p4_cfg_v1::Table;

    fn deref(&self) -> &Self::Target {
        self.table
    }
}

impl<'a> Table<'a> {
    /// Create a new `Table` struct.
    pub fn new(
        table: &'a p4_cfg_v1::Table,
        action_map_full: &'a HashMap<u32, p4_cfg_v1::Action>,
    ) -> Self {
        let mut match_field_map = HashMap::new();
        let mut action_map = HashMap::new();

        for field in table.match_fields.iter() {
            match_field_map.insert(field.name.as_str(), field.clone());
        }

        for action_ref in table.action_refs.iter() {
            let action_id = action_ref.id;
            let action = action_map_full.get(&action_id).unwrap();
            action_map.insert(
                action.preamble.as_ref().unwrap().name.as_str(),
                action.clone(),
            );
            action_map.insert(
                action.preamble.as_ref().unwrap().alias.as_str(),
                action.clone(),
            );
        }

        Table {
            table,
            match_field_map,
            action_map,
        }
    }

    /// Get the table ID.
    pub fn id(&self) -> u32 {
        self.table.preamble.as_ref().map(|p| p.id).unwrap_or(0)
    }

    /// Make a new field match
    pub fn make_field_match(
        &self,
        field_name: impl AsRef<str>,
        value: Vec<u8>,
        second: Option<impl Into<FieldMatchValue>>,
    ) -> Result<p4_v1::FieldMatch, MakeFieldMatchError> {
        use p4_cfg_v1::match_field::{
            Match::*,
            MatchType::{self, *},
        };

        let field_name = field_name.as_ref();

        let field =
            self.match_field_map
                .get(field_name)
                .ok_or(MakeFieldMatchError::UnexistedField {
                    field_name: field_name.to_string(),
                })?;

        let field_match = match field
            .r#match
            .as_ref()
            .ok_or(MakeFieldMatchError::MissingMatchType)?
        {
            MatchType(mt) => {
                let mt = MatchType::try_from(*mt).unwrap();
                match mt {
                    Exact => p4_v1::FieldMatch {
                        field_id: field.id,
                        field_match_type: Some(p4_v1::field_match::FieldMatchType::Exact(
                            p4_v1::field_match::Exact { value },
                        )),
                    },
                    Lpm => {
                        let second = second
                            .ok_or(MakeFieldMatchError::MissingSecondValue)
                            .map(|v| v.into());

                        match second {
                            Ok(FieldMatchValue::I32(prefix_len)) => p4_v1::FieldMatch {
                                field_id: field.id,
                                field_match_type: Some(p4_v1::field_match::FieldMatchType::Lpm(
                                    p4_v1::field_match::Lpm { value, prefix_len },
                                )),
                            },
                            Ok(_) => return Err(MakeFieldMatchError::ExpectedI32),
                            Err(e) => return Err(e),
                        }
                    }
                    Ternary => {
                        let second = second
                            .ok_or(MakeFieldMatchError::MissingSecondValue)
                            .map(|v| v.into());

                        match second {
                            Ok(FieldMatchValue::Vec(mask)) => p4_v1::FieldMatch {
                                field_id: field.id,
                                field_match_type: Some(
                                    p4_v1::field_match::FieldMatchType::Ternary(
                                        p4_v1::field_match::Ternary { value, mask },
                                    ),
                                ),
                            },
                            Ok(_) => return Err(MakeFieldMatchError::ExpectedVec),
                            Err(e) => return Err(e),
                        }
                    }
                    Range => {
                        let second = second
                            .ok_or(MakeFieldMatchError::MissingSecondValue)
                            .map(|v| v.into());

                        match second {
                            Ok(FieldMatchValue::Vec(high)) => p4_v1::FieldMatch {
                                field_id: field.id,
                                field_match_type: Some(p4_v1::field_match::FieldMatchType::Range(
                                    p4_v1::field_match::Range { low: value, high },
                                )),
                            },
                            Ok(_) => return Err(MakeFieldMatchError::ExpectedVec),
                            Err(e) => return Err(e),
                        }
                    }
                    Optional => p4_v1::FieldMatch {
                        field_id: field.id,
                        field_match_type: Some(p4_v1::field_match::FieldMatchType::Optional(
                            p4_v1::field_match::Optional { value },
                        )),
                    },
                    _ => {
                        return Err(MakeFieldMatchError::UnsupportedMatchType {
                            match_type: mt.as_str_name().to_string(),
                        });
                    }
                }
            }
            OtherMatchType(s) => {
                return Err(MakeFieldMatchError::UnsupportedMatchType {
                    match_type: s.clone(),
                });
            }
        };

        Ok(field_match)
    }

    /// Make a new table action
    pub fn make_action(
        &self,
        action_name: impl AsRef<str>,
        params: Vec<Vec<u8>>,
    ) -> Result<p4_v1::TableAction, MakeTableActionError> {
        let action_name = action_name.as_ref();
        let action =
            self.action_map
                .get(action_name)
                .ok_or(MakeTableActionError::UnexistedAction {
                    action_name: action_name.to_string(),
                })?;

        let params = params
            .into_iter()
            .enumerate()
            .map(|(i, param)| p4_v1::action::Param {
                param_id: (i + 1) as u32,
                value: param,
            })
            .collect();

        let action = p4_v1::TableAction {
            r#type: Some(p4_v1::table_action::Type::Action(p4_v1::Action {
                action_id: action.preamble.as_ref().unwrap().id,
                params,
            })),
        };

        Ok(action)
    }

    /// Make a new table entry
    pub fn make_table_entry(
        &self,
        match_fields: Vec<p4_v1::FieldMatch>,
        action: Option<p4_v1::TableAction>,
        priority: i32,
    ) -> p4_v1::TableEntry {
        p4_v1::TableEntry {
            table_id: self.id(),
            r#match: match_fields,
            action,
            priority,

            ..Default::default()
        }
    }
}
