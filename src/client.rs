//! The client wrapper for P4Runtime

use p4runtime::p4::v1 as p4_v1;
use tonic::{codegen::*, transport::Channel};

use crate::{counter::Counter, digest::Digest, p4info::P4Info, table::Table};

/// Client options
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClientOptions {
    /// Whether to canonicalize bytestrings
    pub canonical_bytestrings: bool,

    /// The buffer size for the stream channel
    pub stream_channel_buffer_size: usize,
}

/// P4Runtime client wrapper
#[derive(Debug)]
pub struct Client {
    /// P4Runtime tonic client
    pub p4rt_client: p4_v1::p4_runtime_client::P4RuntimeClient<Channel>,

    /// Device ID
    pub device_id: u64,

    /// Election ID
    pub election_id: p4_v1::Uint128,

    /// Role
    pub role: Option<p4_v1::Role>,

    /// P4Info
    pub p4info: P4Info,

    /// Client options
    pub options: ClientOptions,

    /// Stream Message Sender
    pub stream_message_sender: Option<tokio::sync::mpsc::Sender<p4_v1::StreamMessageRequest>>,

    /// Stream Message Receiver
    pub stream_message_receiver: Option<tonic::Streaming<p4_v1::StreamMessageResponse>>,
}

impl Client {
    /// Create a new client wrapper for P4Runtime
    ///
    /// # Arguments
    ///
    /// - `p4rt_client`: The P4Runtime tonic client
    ///   - This is the inner client that will be used to send and receive messages
    /// - `device_id`: The device ID
    /// - `election_id`: The election ID
    /// - `role`: The role of the client
    /// - `options`: Options of the client wrapper
    pub fn new(
        p4rt_client: p4_v1::p4_runtime_client::P4RuntimeClient<Channel>,
        device_id: u64,
        election_id: p4_v1::Uint128,
        role: Option<p4_v1::Role>,
        options: ClientOptions,
    ) -> Self {
        Self {
            p4rt_client,
            device_id,
            election_id,
            role,
            p4info: P4Info::default(),
            options,

            stream_message_sender: None,
            stream_message_receiver: None,
        }
    }

    /// Get the role name of this client
    pub fn role_name(&self) -> String {
        self.role
            .as_ref()
            .map(|r| r.name.clone())
            .unwrap_or_default()
    }

    /// Get the p4info helper
    pub fn p4info(&self) -> &P4Info {
        &self.p4info
    }

    /// Get the mutable p4info helper
    pub fn p4info_mut(&mut self) -> &mut P4Info {
        &mut self.p4info
    }

    /// Get the table helper
    pub fn table(&self) -> Table<&Self> {
        Table::new(self)
    }

    /// Get the mutable table helper
    pub fn table_mut(&mut self) -> Table<&mut Self> {
        Table::new(self)
    }

    /// Get the counter helper
    pub fn counter(&self) -> Counter<&Self> {
        Counter::new(self)
    }

    /// Get the mutable counter helper
    pub fn counter_mut(&mut self) -> Counter<&mut Self> {
        Counter::new(self)
    }

    /// Get the digest helper
    pub fn digest(&self) -> Digest<&Self> {
        Digest::new(self)
    }

    /// Get the mutable digest helper
    pub fn digest_mut(&mut self) -> Digest<&mut Self> {
        Digest::new(self)
    }
}

impl Client {
    /// Run the client by preparing channels and sending arbitration
    pub async fn run(&mut self) -> Result<(), error::RunError> {
        let (stream_request_sender, stream_request_receiver) =
            tokio::sync::mpsc::channel(self.options.stream_channel_buffer_size);
        self.stream_message_sender = Some(stream_request_sender);

        // start arbitration
        let req = p4_v1::StreamMessageRequest {
            update: Some(p4_v1::stream_message_request::Update::Arbitration(
                p4_v1::MasterArbitrationUpdate {
                    device_id: self.device_id,
                    role: self.role.clone(),
                    election_id: Some(self.election_id.clone()),
                    status: None,
                },
            )),
        };

        self.stream_message_sender
            .as_ref()
            .unwrap()
            .send(req)
            .await?;

        let channel = self
            .p4rt_client
            .stream_channel(tokio_stream::wrappers::ReceiverStream::new(
                stream_request_receiver,
            ))
            .await?
            .into_inner();

        self.stream_message_receiver = Some(channel);

        // Check if the arbitration succeeded
        let res = self
            .stream_message_receiver
            .as_mut()
            .unwrap()
            .message()
            .await?;

        match res {
            Some(p4_v1::StreamMessageResponse {
                update:
                    Some(p4_v1::stream_message_response::Update::Arbitration(
                        p4_v1::MasterArbitrationUpdate {
                            // 0 is p4runtime::google::rpc::Code::Ok
                            // Don't know why can't use `as i32` here
                            status: Some(p4runtime::google::rpc::Status { code: 0, .. }),
                            ..
                        },
                    )),
            }) => Ok(()),
            Some(res) => Err(error::RunError::UnexpectedResponse(res)),
            None => Err(error::RunError::NoneFound),
        }
    }

    /// Set the forwarding pipeline config
    ///
    /// # Arguments
    ///
    /// - `p4_device_config`: The P4 device config
    ///   - This is the binary blob that is generated by the P4 compiler
    ///   - For bmv2, this is the output JSON file
    ///   - For Tofino, this can be built via [`build_tofino_config`](crate::config::build_tofino_config)
    pub async fn set_forwarding_pipeline_config(
        &mut self,
        p4_device_config: Vec<u8>,
    ) -> Result<tonic::Response<p4_v1::SetForwardingPipelineConfigResponse>, tonic::Status> {
        let req = p4_v1::SetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            role: self.role_name(),
            election_id: Some(self.election_id.clone()),
            action: p4_v1::set_forwarding_pipeline_config_request::Action::VerifyAndCommit as i32,
            config: Some(p4_v1::ForwardingPipelineConfig {
                p4info: Some(self.p4info.as_ref().clone()),
                p4_device_config,
                cookie: Some(p4_v1::forwarding_pipeline_config::Cookie { cookie: 0 }),
            }),

            ..Default::default()
        };

        self.p4rt_client.set_forwarding_pipeline_config(req).await
    }

    /// Get the forwarding pipeline config
    pub async fn get_forwarding_pipeline_config(
        &mut self,
        response_type: p4_v1::get_forwarding_pipeline_config_request::ResponseType,
    ) -> Result<tonic::Response<p4_v1::GetForwardingPipelineConfigResponse>, tonic::Status> {
        let req = p4_v1::GetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            response_type: response_type as i32,
        };

        self.p4rt_client.get_forwarding_pipeline_config(req).await
    }

    /// Write a batch of updates
    #[inline]
    pub async fn write_update_batch(
        &mut self,
        updates: Vec<p4_v1::Update>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        let req = p4_v1::WriteRequest {
            device_id: self.device_id,
            role: self.role_name(),
            election_id: Some(self.election_id.clone()),
            updates,

            ..Default::default()
        };

        self.p4rt_client.write(req).await
    }

    /// Write a single update
    #[inline]
    pub async fn write_update(
        &mut self,
        update: p4_v1::Update,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, tonic::Status> {
        self.write_update_batch(vec![update]).await
    }

    /// Read a batch of entities, returning a stream of responses
    #[inline]
    pub async fn read_entity_stream_batch(
        &mut self,
        entities: Vec<p4_v1::Entity>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<p4_v1::ReadResponse>>, tonic::Status> {
        let req = p4_v1::ReadRequest {
            device_id: self.device_id,
            role: self.role_name(),
            entities,
        };

        self.p4rt_client.read(req).await
    }

    /// Read a single entity, returning a stream of responses
    #[inline]
    pub async fn read_entity_stream(
        &mut self,
        entity: p4_v1::Entity,
    ) -> Result<tonic::Response<tonic::codec::Streaming<p4_v1::ReadResponse>>, tonic::Status> {
        self.read_entity_stream_batch(vec![entity]).await
    }

    /// Read exactly one entity
    #[inline]
    pub async fn read_entity_single(
        &mut self,
        entity: p4_v1::Entity,
    ) -> Result<p4_v1::Entity, error::ReadEntitySingleError> {
        let mut stream = self.read_entity_stream(entity).await?.into_inner();

        let mut entities: Vec<p4_v1::Entity> = Vec::new();

        while let Some(res) = stream.message().await? {
            entities.extend(res.entities)
        }

        match entities.len() {
            0 => Err(error::ReadEntitySingleError::NoneFound),
            1 => Ok(entities[0].clone()),
            n @ 2.. => Err(error::ReadEntitySingleError::MultipleFound(n)),
        }
    }

    /// Read all entities
    #[inline]
    pub async fn read_entities(
        &mut self,
        entity: p4_v1::Entity,
    ) -> Result<Vec<p4_v1::Entity>, tonic::Status> {
        let mut stream = self.read_entity_stream(entity).await?.into_inner();

        let mut entities: Vec<p4_v1::Entity> = Vec::new();

        while let Some(res) = stream.message().await? {
            entities.extend(res.entities)
        }

        Ok(entities)
    }

    /// Read a batch of entities
    #[inline]
    pub async fn read_entities_batch(
        &mut self,
        entities: Vec<p4_v1::Entity>,
    ) -> Result<Vec<p4_v1::Entity>, tonic::Status> {
        let mut stream = self.read_entity_stream_batch(entities).await?.into_inner();

        let mut entities: Vec<p4_v1::Entity> = Vec::new();

        while let Some(res) = stream.message().await? {
            entities.extend(res.entities)
        }

        Ok(entities)
    }

    /// Send a stream message request
    #[inline]
    pub async fn send_message_request(
        &mut self,
        request: p4_v1::StreamMessageRequest,
    ) -> Result<(), error::SendStreamMessageRequestError> {
        Ok(self
            .stream_message_sender
            .as_ref()
            .unwrap()
            .send(request)
            .await?)
    }
}

/// Error types
pub mod error {
    use p4runtime::p4::v1 as p4_v1;
    use thiserror::Error;

    /// Error for [`run`](crate::client::Client::run) method
    #[derive(Debug, Error)]
    pub enum RunError {
        /// Error for sending stream message request
        #[error("Tokio Send Error: {0}")]
        SendError(#[from] tokio::sync::mpsc::error::SendError<p4_v1::StreamMessageRequest>),

        /// Error for receiving rpc response
        #[error("Tonic status: {0}")]
        TonicStatus(#[from] tonic::Status),

        /// Error for unexpected stream message response
        #[error("Expected arbitration response, got {0:?}")]
        UnexpectedResponse(p4_v1::StreamMessageResponse),

        /// Error for no arbitration response
        #[error("Expected arbitration response, got none")]
        NoneFound,
    }

    /// Error for [`send_message_request`](crate::client::Client::send_message_request) method
    #[derive(Debug, Error)]
    pub enum SendStreamMessageRequestError {
        /// Error for sending stream message request
        #[error("Tokio Send Error: {0}")]
        SendError(#[from] tokio::sync::mpsc::error::SendError<p4_v1::StreamMessageRequest>),
    }

    /// Error for [`read_entity_single`](crate::client::Client::read_entity_single) method
    #[derive(Debug, Error)]
    pub enum ReadEntitySingleError {
        /// Error for receiving rpc response
        #[error("Tonic status: {0}")]
        TonicStatus(#[from] tonic::Status),

        /// Error for no entity found
        #[error("Expected exactly one entity, got none")]
        NoneFound,

        /// Error for multiple entities found
        #[error("Expected exactly one entity, got {0}")]
        MultipleFound(usize),
    }
}
