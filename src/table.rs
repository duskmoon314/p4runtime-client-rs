use std::borrow::{Borrow, BorrowMut};

use p4runtime::p4::v1 as p4_v1;

use crate::client::Client;

/// Wrapper for table operations
pub struct Table<T>
where
    T: Borrow<Client>,
{
    client: T,
}

impl<T: Borrow<Client>> Table<T> {
    /// Create a new table wrapper
    pub fn new(client: T) -> Self {
        Table { client }
    }

    pub fn new_action(&self, action_name: &str, params: Vec<Vec<u8>>) -> p4_v1::TableAction {
        let client: &Client = self.client.borrow();
        let action_id = client.p4info().action_id(action_name);

        let params = params
            .into_iter()
            .enumerate()
            .map(|(i, param)| p4_v1::action::Param {
                param_id: (i + 1) as u32,
                value: param,
            })
            .collect::<Vec<_>>();

        p4_v1::TableAction {
            r#type: Some(p4_v1::table_action::Type::Action(p4_v1::Action {
                action_id,
                params,
            })),
        }
    }

    pub fn new_entry(
        &self,
        table_name: &str,
        match_fields: Vec<(String, p4_v1::field_match::FieldMatchType)>,
        action: Option<p4_v1::TableAction>,
        priority: i32,
    ) -> p4_v1::TableEntry {
        let client: &Client = self.client.borrow();
        let table_id = client.p4info().table_id(table_name);
        let match_fields = match_fields
            .into_iter()
            .map(|(match_field_name, field_match_type)| p4_v1::FieldMatch {
                field_id: client
                    .p4info()
                    .table_match_field_id(table_name, &match_field_name),
                field_match_type: Some(field_match_type),
            })
            .collect();

        p4_v1::TableEntry {
            table_id,
            r#match: match_fields,
            action,
            priority,

            ..Default::default()
        }
    }
}

impl<T: Borrow<Client> + BorrowMut<Client>> Table<T> {
    pub async fn read_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<p4_v1::TableEntry, error::ReadTableEntrySingleError> {
        let entity = p4_v1::Entity {
            entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
        };

        let client: &mut Client = self.client.borrow_mut();
        let entity = client.read_entity_single(entity).await?;

        if let Some(p4_v1::entity::Entity::TableEntry(table_entry)) = entity.entity {
            Ok(table_entry)
        } else {
            Err(error::ReadTableEntrySingleError::NotTableEntry(
                entity.entity,
            ))
        }
    }

    pub async fn read_entries(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<Vec<p4_v1::TableEntry>, error::ReadTableEntriesError> {
        let entity = p4_v1::Entity {
            entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
        };

        let client: &mut Client = self.client.borrow_mut();
        let entities = client.read_entities(entity).await?;

        let entries = entities
            .into_iter()
            .map(|e| {
                if let Some(p4_v1::entity::Entity::TableEntry(table_entry)) = e.entity {
                    Ok(table_entry)
                } else {
                    Err(error::ReadTableEntriesError::NotTableEntry(e.entity))
                }
            })
            .collect::<Result<Vec<p4_v1::TableEntry>, error::ReadTableEntriesError>>()?;

        Ok(entries)
    }

    pub async fn insert_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Insert as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    pub async fn insert_entries(
        &mut self,
        table_entries: Vec<p4_v1::TableEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let updates = table_entries
            .into_iter()
            .map(|table_entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Insert as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        client.write_update_batch(updates).await
    }

    pub async fn modify_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Modify as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    pub async fn modify_entries(
        &mut self,
        table_entries: Vec<p4_v1::TableEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let updates = table_entries
            .into_iter()
            .map(|table_entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Modify as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        client.write_update_batch(updates).await
    }

    pub async fn delete_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Delete as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    pub async fn delete_entries(
        &mut self,
        table_entries: Vec<p4_v1::TableEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let updates = table_entries
            .into_iter()
            .map(|table_entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Delete as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        client.write_update_batch(updates).await
    }
}

pub mod error {
    use p4runtime::p4::v1 as p4_v1;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ReadTableEntrySingleError {
        #[error(transparent)]
        ReadEntitySingle(#[from] crate::client::error::ReadEntitySingleError),

        #[error("Entity is not a TableEntry: {0:?}")]
        NotTableEntry(Option<p4_v1::entity::Entity>),
    }

    #[derive(Error, Debug)]
    pub enum ReadTableEntriesError {
        #[error("Tonic status: {0}")]
        TonicStatus(#[from] tonic::Status),

        #[error("Entity is not a TableEntry: {0:?}")]
        NotTableEntry(Option<p4_v1::entity::Entity>),
    }
}
