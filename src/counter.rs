//! Counter helper and operations

use std::borrow::{Borrow, BorrowMut};

use p4runtime::p4::v1 as p4_v1;

use crate::{client::Client, error::ClientError};

/// Wrapper for counter operations
pub struct Counter<T>
where
    T: Borrow<Client>,
{
    client: T,
}

impl<T: Borrow<Client>> Counter<T> {
    /// Create a new counter wrapper
    pub fn new(client: T) -> Self {
        Counter { client }
    }

    /// Create a new CounterEntry by name
    ///
    /// # Arguments
    ///
    /// - `counter_name`: The name of the counter
    ///   - It is used to find the counter id in P4Info
    ///   - If the name is not found, wildcard is used, i.e., id = 0
    pub fn new_entry(
        &self,
        counter_name: &str,
        index: Option<i64>,
        data: Option<p4_v1::CounterData>,
    ) -> p4_v1::CounterEntry {
        let client = self.client.borrow();
        let counter_id = client.p4info().counter_id(counter_name);

        p4_v1::CounterEntry {
            counter_id,
            index: index.map(|i| p4_v1::Index { index: i }),
            data,
        }
    }
}

impl<T: Borrow<Client> + BorrowMut<Client>> Counter<T> {
    /// Read a single counter entry
    pub async fn read_entry(
        &mut self,
        counter_entry: p4_v1::CounterEntry,
    ) -> Result<p4_v1::CounterEntry, ClientError> {
        let entity = p4_v1::Entity {
            entity: Some(p4_v1::entity::Entity::CounterEntry(counter_entry)),
        };

        let client: &mut Client = self.client.borrow_mut();
        let entity = client.read_entity_single(entity).await?;

        if let Some(p4_v1::entity::Entity::CounterEntry(counter_entry)) = entity.entity {
            Ok(counter_entry)
        } else {
            Err(ClientError::UnexpectedEntry)
        }
    }

    /// Read multiple counter entries
    pub async fn read_entries(
        &mut self,
        counter_entry: p4_v1::CounterEntry,
    ) -> Result<Vec<p4_v1::CounterEntry>, ClientError> {
        let entity = p4_v1::Entity {
            entity: Some(p4_v1::entity::Entity::CounterEntry(counter_entry)),
        };

        let client: &mut Client = self.client.borrow_mut();
        let entities = client.read_entities(entity).await?;

        let entries = entities
            .into_iter()
            .map(|e| {
                if let Some(p4_v1::entity::Entity::CounterEntry(counter_entry)) = e.entity {
                    Ok(counter_entry)
                } else {
                    Err(ClientError::UnexpectedEntry)
                }
            })
            .collect::<Result<Vec<p4_v1::CounterEntry>, ClientError>>()?;

        Ok(entries)
    }

    /// Read multiple counters' entries
    pub async fn read_entries_batch(
        &mut self,
        counter_entries: Vec<p4_v1::CounterEntry>,
    ) -> Result<Vec<p4_v1::CounterEntry>, ClientError> {
        let entities = counter_entries
            .into_iter()
            .map(|counter_entry| p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::CounterEntry(counter_entry)),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        let entities = client.read_entities_batch(entities).await?;

        let entries = entities
            .into_iter()
            .map(|e| {
                if let Some(p4_v1::entity::Entity::CounterEntry(counter_entry)) = e.entity {
                    Ok(counter_entry)
                } else {
                    Err(ClientError::UnexpectedEntry)
                }
            })
            .collect::<Result<Vec<p4_v1::CounterEntry>, ClientError>>()?;

        Ok(entries)
    }

    /// Modify a counter entry
    pub async fn modify_entry(
        &mut self,
        counter_entry: p4_v1::CounterEntry,
    ) -> Result<p4_v1::WriteResponse, ClientError> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Modify as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::CounterEntry(counter_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        let res = client.write_update(update).await?;
        Ok(res.into_inner())
    }

    /// Modify multiple counter entries
    pub async fn modify_entries(
        &mut self,
        counter_entries: Vec<p4_v1::CounterEntry>,
    ) -> Result<p4_v1::WriteResponse, ClientError> {
        let updates = counter_entries
            .into_iter()
            .map(|counter_entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Modify as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::CounterEntry(counter_entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        let res = client.write_update_batch(updates).await?;
        Ok(res.into_inner())
    }
}
