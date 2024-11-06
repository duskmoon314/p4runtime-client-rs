use p4runtime_client::client::Client;
use p4runtime_client::p4runtime::p4::config::v1::P4Info;
use p4runtime_client::p4runtime::p4::v1::{self as p4_v1, Uint128};
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
    env_logger::init();

    let p4bin = include_bytes!("../build/main.json");
    let p4info = include_bytes!("../build/main.p4info.bin");
    let p4info = P4Info::decode(&p4info[..])?;

    let mut client = Client::builder()
        .device_id(0)
        .election_id(Uint128 { high: 0, low: 1 })
        .build()?;
    client.connect("http://127.0.0.1:9559").await?;

    let capabilities = client.capabilities().await?;
    println!(
        "Server P4Runtime capabilities: {:?}",
        capabilities.p4runtime_api_version
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

    while let Ok(digest) = client.get_digest(1).await {
        let data = digest.data;
        for d in data {
            let ipv4_digest: Ipv4Digest = from_p4data(&d)?;
            println!("Received IPv4 digest: {:?}", ipv4_digest);
        }
    }

    Ok(())
}
