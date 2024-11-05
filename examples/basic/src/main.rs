use p4runtime_client::client::{Client, ClientOptions};
use p4runtime_client::p4runtime::p4::config::v1::P4Info;
use p4runtime_client::p4runtime::p4::v1::p4_runtime_client::P4RuntimeClient;
use p4runtime_client::p4runtime::p4::v1::stream_message_response::Update;
use p4runtime_client::p4runtime::p4::v1::{
    self as p4_v1, CapabilitiesRequest, DigestList, Uint128,
};
use p4runtime_client::utils::de::from_p4data;
use prost::Message;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct Ipv4Digest {
    dst_mac: [u8; 6],
    dst_ip: std::net::Ipv4Addr,
    ip_proto: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let p4bin = include_bytes!("../build/main.json");
    let p4info = include_bytes!("../build/main.p4info.bin");
    let p4info = P4Info::decode(&p4info[..])?;

    let mut p4rt_client = P4RuntimeClient::connect("http://127.0.0.1:9559").await?;
    let res = p4rt_client.capabilities(CapabilitiesRequest {}).await?;
    println!(
        "Server P4Runtime capabilities: {:?}",
        res.get_ref().p4runtime_api_version
    );

    let mut client = Client::new(
        p4rt_client,
        0,
        Uint128 { high: 0, low: 1 },
        None,
        ClientOptions {
            stream_channel_buffer_size: 1024,

            ..Default::default()
        },
    );

    println!("Starting master arbitration...");
    client.run().await?;

    client.p4info_mut().load(p4info);

    println!("Setting pipeline config...");
    client
        .set_forwarding_pipeline_config(p4bin.to_vec())
        .await?;

    let table_entry_1 = client.table().new_entry(
        "ipv4_lpm",
        vec![(
            "hdr.ipv4.dst".to_string(),
            p4_v1::field_match::FieldMatchType::Lpm(p4_v1::field_match::Lpm {
                value: vec![10, 0, 1, 2],
                prefix_len: 32,
            }),
        )],
        Some(
            client
                .table()
                .new_action("ipv4_forward", vec![vec![8, 0, 0, 0, 1, 2], vec![1]]),
        ),
        0,
    );
    let table_entry_2 = client.table().new_entry(
        "ipv4_lpm",
        vec![(
            "hdr.ipv4.dst".to_string(),
            p4_v1::field_match::FieldMatchType::Lpm(p4_v1::field_match::Lpm {
                value: vec![10, 0, 2, 2],
                prefix_len: 32,
            }),
        )],
        Some(
            client
                .table()
                .new_action("ipv4_forward", vec![vec![8, 0, 0, 0, 2, 2], vec![2]]),
        ),
        0,
    );

    client
        .table_mut()
        .insert_entries(vec![table_entry_1, table_entry_2])
        .await?;

    println!("Table entries inserted.");

    let digest_entry = client.digest().new_entry("ipv4_digest_t", 0, 1, 0);
    client.digest_mut().insert_entry(digest_entry).await?;

    while let Some(msg) = client
        .stream_message_receiver
        .as_mut()
        .unwrap()
        .message()
        .await?
    {
        if let Some(Update::Digest(DigestList { data, .. })) = msg.update {
            for d in data {
                let ipv4_digest: Ipv4Digest = from_p4data(&d)?;
                println!("Received IPv4 digest: {:?}", ipv4_digest);
            }
        }
    }

    Ok(())
}
