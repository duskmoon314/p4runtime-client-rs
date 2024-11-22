//! The client wrapper for P4Runtime

use log::{debug, error, info, warn};
use p4runtime::p4::v1::{self as p4_v1, p4_runtime_client::P4RuntimeClient};
use tokio_util::sync::CancellationToken;
use tonic::{codegen::*, transport::Channel};

use crate::{counter::Counter, digest::Digest, error::ClientError, p4info::P4Info, table::Table};

/// P4Runtime client wrapper
#[derive(Debug, Default, derive_builder::Builder)]
#[builder(default)]
pub struct Client {
    /// P4Runtime tonic client
    pub p4rt_client: Option<p4_v1::p4_runtime_client::P4RuntimeClient<Channel>>,

    /// Device ID
    pub device_id: u64,

    /// Election ID
    pub election_id: p4_v1::Uint128,

    /// Role
    pub role: Option<p4_v1::Role>,

    /// P4Info
    pub p4info: P4Info,

    /// stream channel buffer size
    #[builder(default = 10000)]
    pub channel_buffer_size: usize,

    /// cancel token
    ///
    /// This is used to cancel inner threads
    #[builder(setter(skip))]
    cancel_token: CancellationToken,

    /// stream message request sender
    #[builder(setter(skip))]
    stream_message_sender: Option<tokio::sync::mpsc::Sender<p4_v1::StreamMessageRequest>>,

    #[builder(setter(skip))]
    arbitration_rx: Option<tokio::sync::broadcast::Receiver<p4_v1::MasterArbitrationUpdate>>,

    #[builder(setter(skip))]
    packet_rx: Option<tokio::sync::broadcast::Receiver<p4_v1::PacketIn>>,

    #[builder(setter(skip))]
    digest_rx: Option<tokio::sync::broadcast::Receiver<p4_v1::DigestList>>,

    #[builder(setter(skip))]
    idle_timeout_rx: Option<tokio::sync::broadcast::Receiver<p4_v1::IdleTimeoutNotification>>,

    #[builder(setter(skip))]
    error_rx: Option<tokio::sync::broadcast::Receiver<p4_v1::StreamError>>,
}

impl Client {
    /// Create the client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Connect to the P4Runtime server
    pub async fn connect<D>(&mut self, dst: D) -> Result<(), tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let p4rt_client = P4RuntimeClient::connect(dst).await?;
        self.p4rt_client = Some(p4rt_client);
        Ok(())
    }

    /// Get the role name of this client
    pub fn role_name(&self) -> Option<String> {
        self.role.as_ref().map(|r| r.name.clone())
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

    /// Quit the client
    pub async fn quit(&mut self) {
        info!("Quitting P4Runtime client");
        self.cancel_token.cancel();
    }

    /// Is the client cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.is_cancelled()
    }

    /// Send a stream message request
    pub async fn send_message_request(
        &mut self,
        request: p4_v1::StreamMessageRequest,
    ) -> Result<(), ClientError> {
        Ok(self
            .stream_message_sender
            .as_ref()
            .unwrap()
            .send(request)
            .await?)
    }

    /// Subscribe to arbitration updates
    pub fn subscribe_arbitration(
        &self,
    ) -> tokio::sync::broadcast::Receiver<p4_v1::MasterArbitrationUpdate> {
        self.arbitration_rx.as_ref().unwrap().resubscribe()
    }

    /// Get an arbitration
    pub async fn get_arbitration(
        &mut self,
        timeout: u64,
    ) -> Result<p4_v1::MasterArbitrationUpdate, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientError::Timeout)
            }

            arb = self.arbitration_rx.as_mut().unwrap().recv() => {
                Ok(arb.unwrap())
            }
        }
    }

    /// Subscribe to packet in messages
    pub fn subscribe_packet_in(&self) -> tokio::sync::broadcast::Receiver<p4_v1::PacketIn> {
        self.packet_rx.as_ref().unwrap().resubscribe()
    }

    /// Get a packet in message
    pub async fn get_packet_in(&mut self, timeout: u64) -> Result<p4_v1::PacketIn, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientError::Timeout)
            }

            packet = self.packet_rx.as_mut().unwrap().recv() => {
                Ok(packet.unwrap())
            }
        }
    }

    /// Subscribe to digest messages
    pub fn subscribe_digest(&self) -> tokio::sync::broadcast::Receiver<p4_v1::DigestList> {
        self.digest_rx.as_ref().unwrap().resubscribe()
    }

    /// Get a digest message
    pub async fn get_digest(&mut self, timeout: u64) -> Result<p4_v1::DigestList, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientError::Timeout)
            }

            digest = self.digest_rx.as_mut().unwrap().recv() => {
                Ok(digest.unwrap())
            }
        }
    }

    /// Subscribe to idle timeout notifications
    pub fn subscribe_idle_timeout(
        &self,
    ) -> tokio::sync::broadcast::Receiver<p4_v1::IdleTimeoutNotification> {
        self.idle_timeout_rx.as_ref().unwrap().resubscribe()
    }

    /// Get an idle timeout notification
    pub async fn get_idle_timeout(
        &mut self,
        timeout: u64,
    ) -> Result<p4_v1::IdleTimeoutNotification, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientError::Timeout)
            }

            idle_timeout = self.idle_timeout_rx.as_mut().unwrap().recv() => {
                Ok(idle_timeout.unwrap())
            }
        }
    }

    /// Subscribe to stream errors
    pub fn subscribe_error(&self) -> tokio::sync::broadcast::Receiver<p4_v1::StreamError> {
        self.error_rx.as_ref().unwrap().resubscribe()
    }

    /// Get a stream error
    pub async fn get_error(&mut self, timeout: u64) -> Result<p4_v1::StreamError, ClientError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientError::Timeout)
            }

            error = self.error_rx.as_mut().unwrap().recv() => {
                Ok(error.unwrap())
            }
        }
    }

    /// Rund the client by preparing channels and sending arbitration
    pub async fn run(&mut self) -> Result<(), ClientError> {
        info!("Running P4Runtime client");
        debug!(
            "Client config: Did {}, Eid {:?}, Role {:?}",
            self.device_id,
            self.election_id,
            self.role_name()
        );

        debug!("Setting up stream message channel");

        let (stream_request_sender, stream_request_receiver) =
            tokio::sync::mpsc::channel(self.channel_buffer_size);
        self.stream_message_sender = Some(stream_request_sender);

        debug!("Sending arbitration request");

        // start arbitration
        let req = p4_v1::StreamMessageRequest {
            update: Some(p4_v1::stream_message_request::Update::Arbitration(
                p4_v1::MasterArbitrationUpdate {
                    device_id: self.device_id,
                    role: self.role.clone(),
                    election_id: Some(self.election_id),
                    status: None,
                },
            )),
        };

        self.send_message_request(req).await?;

        debug!("Creating stream channel between client and server");

        let channel = self
            .p4rt_client
            .as_mut()
            .ok_or(ClientError::MissingP4rtClient)?
            .stream_channel(tokio_stream::wrappers::ReceiverStream::new(
                stream_request_receiver,
            ))
            .await?
            .into_inner();

        debug!("Setting up broadcast channels");

        let (arbitration_tx, arbitration_rx) =
            tokio::sync::broadcast::channel(self.channel_buffer_size);
        let (packet_tx, packet_rx) = tokio::sync::broadcast::channel(self.channel_buffer_size);
        let (digest_tx, digest_rx) = tokio::sync::broadcast::channel(self.channel_buffer_size);
        let (idle_timeout_tx, idle_timeout_rx) =
            tokio::sync::broadcast::channel(self.channel_buffer_size);
        let (error_tx, error_rx) = tokio::sync::broadcast::channel(self.channel_buffer_size);

        self.arbitration_rx = Some(arbitration_rx);
        self.packet_rx = Some(packet_rx);
        self.digest_rx = Some(digest_rx);
        self.idle_timeout_rx = Some(idle_timeout_rx);
        self.error_rx = Some(error_rx);
        self.set_up_stream_message_channel(
            channel,
            arbitration_tx,
            packet_tx,
            digest_tx,
            idle_timeout_tx,
            error_tx,
        );

        // Check if arbitration is successful
        let res = self.get_arbitration(5).await?;
        if let Some(status) = res.status {
            if status.code == p4runtime::google::rpc::Code::Ok as i32 {
                Ok(())
            } else {
                Err(ClientError::ArbitrationFailed)
            }
        } else {
            Err(ClientError::ArbitrationFailed)
        }
    }

    fn set_up_stream_message_channel(
        &mut self,
        mut channel: tonic::codec::Streaming<p4_v1::StreamMessageResponse>,
        arbitration_tx: tokio::sync::broadcast::Sender<p4_v1::MasterArbitrationUpdate>,
        packet_tx: tokio::sync::broadcast::Sender<p4_v1::PacketIn>,
        digest_tx: tokio::sync::broadcast::Sender<p4_v1::DigestList>,
        idle_timeout_tx: tokio::sync::broadcast::Sender<p4_v1::IdleTimeoutNotification>,
        error_tx: tokio::sync::broadcast::Sender<p4_v1::StreamError>,
    ) {
        let cancel_token = self.cancel_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => {
                        break;
                    }

                    msg = channel.message() => {
                        match msg {
                            Ok(Some(res)) => {
                                use p4_v1::stream_message_response::Update;

                                // TODO: check if the update could be none
                                let update = res.update.unwrap();

                                // TODO: handle send error
                                match update {
                                    Update::Arbitration(arb) => {
                                        arbitration_tx.send(arb).unwrap();
                                    }
                                    Update::Packet(packet) => {
                                        packet_tx.send(packet).unwrap();
                                    }
                                    Update::Digest(digest) => {
                                        digest_tx.send(digest).unwrap();
                                    }
                                    Update::IdleTimeoutNotification(idle_timeout) => {
                                        idle_timeout_tx.send(idle_timeout).unwrap();
                                    }
                                    Update::Error(error) => {
                                        error_tx.send(error).unwrap();
                                    }
                                    _ => {
                                        warn!("Received unsupported stream message update: {:?}", update);
                                    }
                                }
                            }

                            Ok(None) => {
                                debug!("Stream channel closed");
                                break;
                            }

                            Err(e) => {
                                error!("Channel receive rpc error: {:?}", e);
                                cancel_token.cancel();
                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    /// Get the capabilities
    pub async fn capabilities(&mut self) -> Result<p4_v1::CapabilitiesResponse, ClientError> {
        let req = p4_v1::CapabilitiesRequest {};

        Ok(self
            .p4rt_client
            .as_mut()
            .ok_or(ClientError::MissingP4rtClient)?
            .capabilities(req)
            .await?
            .into_inner())
    }

    /// Get the forwarding pipeline config
    pub async fn get_forwarding_pipeline_config(
        &mut self,
        response_type: p4_v1::get_forwarding_pipeline_config_request::ResponseType,
    ) -> Result<tonic::Response<p4_v1::GetForwardingPipelineConfigResponse>, ClientError> {
        let req = p4_v1::GetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            response_type: response_type as i32,
        };

        Ok(self
            .p4rt_client
            .as_mut()
            .ok_or(ClientError::MissingP4rtClient)?
            .get_forwarding_pipeline_config(req)
            .await?)
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
    ) -> Result<tonic::Response<p4_v1::SetForwardingPipelineConfigResponse>, ClientError> {
        let req = p4_v1::SetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            role: self.role_name().unwrap_or_default(),
            election_id: Some(self.election_id),
            action: p4_v1::set_forwarding_pipeline_config_request::Action::VerifyAndCommit as i32,
            config: Some(p4_v1::ForwardingPipelineConfig {
                p4info: Some(self.p4info.as_ref().clone()),
                p4_device_config,
                cookie: Some(p4_v1::forwarding_pipeline_config::Cookie { cookie: 0 }),
            }),

            ..Default::default()
        };

        Ok(self
            .p4rt_client
            .as_mut()
            .ok_or(ClientError::MissingP4rtClient)?
            .set_forwarding_pipeline_config(req)
            .await?)
    }

    /// Write a batch of updates
    #[inline]
    pub async fn write_update_batch(
        &mut self,
        updates: Vec<p4_v1::Update>,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
        let req = p4_v1::WriteRequest {
            device_id: self.device_id,
            role: self.role_name().unwrap_or_default(),
            election_id: Some(self.election_id),
            updates,

            ..Default::default()
        };

        Ok(self
            .p4rt_client
            .as_mut()
            .ok_or(ClientError::MissingP4rtClient)?
            .write(req)
            .await?)
    }

    /// Write a single update
    #[inline]
    pub async fn write_update(
        &mut self,
        update: p4_v1::Update,
    ) -> Result<tonic::Response<p4_v1::WriteResponse>, ClientError> {
        self.write_update_batch(vec![update]).await
    }

    /// Read a batch of entities, returning a stream of responses
    #[inline]
    pub async fn read_entity_stream_batch(
        &mut self,
        entities: Vec<p4_v1::Entity>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<p4_v1::ReadResponse>>, ClientError> {
        let req = p4_v1::ReadRequest {
            device_id: self.device_id,
            role: self.role_name().unwrap_or_default(),
            entities,
        };

        Ok(self
            .p4rt_client
            .as_mut()
            .ok_or(ClientError::MissingP4rtClient)?
            .read(req)
            .await?)
    }

    /// Read a single entity, returning a stream of responses
    #[inline]
    pub async fn read_entity_stream(
        &mut self,
        entity: p4_v1::Entity,
    ) -> Result<tonic::Response<tonic::codec::Streaming<p4_v1::ReadResponse>>, ClientError> {
        self.read_entity_stream_batch(vec![entity]).await
    }

    /// Read exactly one entity
    #[inline]
    pub async fn read_entity_single(
        &mut self,
        entity: p4_v1::Entity,
    ) -> Result<p4_v1::Entity, ClientError> {
        let mut stream = self.read_entity_stream(entity).await?.into_inner();

        let mut entities: Vec<p4_v1::Entity> = Vec::new();

        while let Some(res) = stream.message().await? {
            entities.extend(res.entities)
        }

        match entities.len() {
            0 => Err(ClientError::NoneEntity),
            1 => Ok(entities[0].clone()),
            n @ 2.. => Err(ClientError::MultipleEntities { n }),
        }
    }

    /// Read all entities
    #[inline]
    pub async fn read_entities(
        &mut self,
        entity: p4_v1::Entity,
    ) -> Result<Vec<p4_v1::Entity>, ClientError> {
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
    ) -> Result<Vec<p4_v1::Entity>, ClientError> {
        let mut stream = self.read_entity_stream_batch(entities).await?.into_inner();

        let mut entities: Vec<p4_v1::Entity> = Vec::new();

        while let Some(res) = stream.message().await? {
            entities.extend(res.entities)
        }

        Ok(entities)
    }
}
