//! Table helper and operations

use std::borrow::{Borrow, BorrowMut};

use p4runtime::p4::v1 as p4_v1;

use crate::{client::Client, error::ClientError};

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

    /// Create a new table action by name and parameters
    ///
    /// # Arguments
    ///
    /// - `action_name`: Name of the action
    ///   - It is used to the action id in P4Info
    ///   - If the action name is not found, wildcard is used, i.e., id = 0
    /// - `params`: Parameters of the action
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

    /// Create a new table entry by table name, match fields, action, and priority
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
    /// Read a table entry
    pub async fn read_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<p4_v1::TableEntry, ClientError> {
        let entity = p4_v1::Entity {
            entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
        };

        let client: &mut Client = self.client.borrow_mut();
        let entity = client.read_entity_single(entity).await?;

        if let Some(p4_v1::entity::Entity::TableEntry(table_entry)) = entity.entity {
            Ok(table_entry)
        } else {
            Err(ClientError::UnexpectedEntry)
        }
    }

    /// Read table entries
    pub async fn read_entries(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<Vec<p4_v1::TableEntry>, ClientError> {
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
                    Err(ClientError::UnexpectedEntry)
                }
            })
            .collect::<Result<Vec<p4_v1::TableEntry>, ClientError>>()?;

        Ok(entries)
    }

    /// Insert a table entry
    pub async fn insert_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Insert as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    /// Insert table entries
    pub async fn insert_entries(
        &mut self,
        table_entries: Vec<p4_v1::TableEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
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

    /// Modify a table entry
    pub async fn modify_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Modify as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    /// Modify table entries
    pub async fn modify_entries(
        &mut self,
        table_entries: Vec<p4_v1::TableEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
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

    /// Delete a table entry
    pub async fn delete_entry(
        &mut self,
        table_entry: p4_v1::TableEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Delete as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    /// Delete table entries
    pub async fn delete_entries(
        &mut self,
        table_entries: Vec<p4_v1::TableEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
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
