//! Digest helper and operations

use std::borrow::{Borrow, BorrowMut};

use p4runtime::p4::v1 as p4_v1;

use crate::client::Client;

/// Wrapper for digest operations
pub struct Digest<T>
where
    T: Borrow<Client>,
{
    client: T,
}

impl<T: Borrow<Client>> Digest<T> {
    /// Create a new Digest wrapper
    pub fn new(client: T) -> Self {
        Digest { client }
    }

    /// Create a new DigestEntry by name
    pub fn new_entry(
        &self,
        digest_name: &str,
        max_timeout_ns: i64,
        max_list_size: i32,
        ack_timeout_ns: i64,
    ) -> p4_v1::DigestEntry {
        let client: &Client = self.client.borrow();
        let digest_id = client.p4info().digest_id(digest_name);

        p4_v1::DigestEntry {
            digest_id,
            config: Some(p4_v1::digest_entry::Config {
                max_timeout_ns,
                max_list_size,
                ack_timeout_ns,
            }),
        }
    }
}

impl<T: Borrow<Client> + BorrowMut<Client>> Digest<T> {
    /// Read a DigestEntry
    pub async fn read_entry(
        &mut self,
        digest_entry: p4_v1::DigestEntry,
    ) -> Result<p4_v1::DigestEntry, error::ReadDigestEntrySingleError> {
        let entity = p4_v1::Entity {
            entity: Some(p4_v1::entity::Entity::DigestEntry(digest_entry)),
        };

        let client: &mut Client = self.client.borrow_mut();
        let entity = client.read_entity_single(entity).await?;

        match entity.entity {
            Some(p4_v1::entity::Entity::DigestEntry(entry)) => Ok(entry),
            _ => Err(error::ReadDigestEntrySingleError::NotDigestEntry(
                entity.entity,
            )),
        }
    }

    /// Insert a DigestEntry
    pub async fn insert_entry(
        &mut self,
        digest_entry: p4_v1::DigestEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Insert as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::DigestEntry(digest_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    /// Insert multiple DigestEntries
    pub async fn insert_entries(
        &mut self,
        digest_entries: Vec<p4_v1::DigestEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let updates = digest_entries
            .into_iter()
            .map(|entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Insert as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::DigestEntry(entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        client.write_update_batch(updates).await
    }

    /// Modify a DigestEntry
    pub async fn modify_entry(
        &mut self,
        digest_entry: p4_v1::DigestEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Modify as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::DigestEntry(digest_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    /// Modify multiple DigestEntries
    pub async fn modify_entries(
        &mut self,
        digest_entries: Vec<p4_v1::DigestEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let updates = digest_entries
            .into_iter()
            .map(|entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Modify as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::DigestEntry(entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        client.write_update_batch(updates).await
    }

    /// Delete a DigestEntry
    pub async fn delete_entry(
        &mut self,
        digest_entry: p4_v1::DigestEntry,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let update = p4_v1::Update {
            r#type: p4_v1::update::Type::Delete as i32,
            entity: Some(p4_v1::Entity {
                entity: Some(p4_v1::entity::Entity::DigestEntry(digest_entry)),
            }),
        };

        let client: &mut Client = self.client.borrow_mut();
        client.write_update(update).await
    }

    /// Delete multiple DigestEntries
    pub async fn delete_entries(
        &mut self,
        digest_entries: Vec<p4_v1::DigestEntry>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let updates = digest_entries
            .into_iter()
            .map(|entry| p4_v1::Update {
                r#type: p4_v1::update::Type::Delete as i32,
                entity: Some(p4_v1::Entity {
                    entity: Some(p4_v1::entity::Entity::DigestEntry(entry)),
                }),
            })
            .collect();

        let client: &mut Client = self.client.borrow_mut();
        client.write_update_batch(updates).await
    }

    /// Acknowledge a DigestList
    pub async fn ack_digest_list(
        &mut self,
        digest_list: &p4_v1::DigestList,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<p4_v1::StreamMessageRequest>> {
        let req = p4_v1::StreamMessageRequest {
            update: Some(p4_v1::stream_message_request::Update::DigestAck(
                p4_v1::DigestListAck {
                    digest_id: digest_list.digest_id,
                    list_id: digest_list.list_id,
                },
            )),
        };

        let client: &mut Client = self.client.borrow_mut();
        client
            .stream_message_sender
            .as_mut()
            .unwrap()
            .send(req)
            .await
    }
}

/// Error types for Digest operations
pub mod error {
    use p4runtime::p4::v1 as p4_v1;
    use thiserror::Error;

    /// Error for [`read_entry`](crate::digest::Digest::read_entry)
    #[derive(Error, Debug)]
    pub enum ReadDigestEntrySingleError {
        /// The inner read entity error
        #[error(transparent)]
        ReadEntitySingle(#[from] crate::client::error::ReadEntitySingleError),

        /// The entity is not a DigestEntry
        #[error("Entity is not a DigestEntry: {0:?}")]
        NotDigestEntry(Option<p4_v1::entity::Entity>),
    }
}
