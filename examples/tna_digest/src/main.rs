use p4runtime_client::{
    client::Client,
    config::build_tofino_config,
    p4runtime::p4::{config::v1 as p4_cfg_v1, v1::Uint128},
    utils::de::from_p4data,
};
use prost::Message;
use tokio::{select, signal, time::sleep, time::Duration};

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

    let mut client = Client::builder()
        .device_id(0)
        .election_id(Uint128 { high: 0, low: 1 })
        .build()?;
    client.connect("http://127.0.0.1:9559").await?;

    println!("Starting master arbitration...");
    client.run().await?;

    let p4info = include_bytes!("../build/p4rt.bin");
    let p4info = p4_cfg_v1::P4Info::decode(&p4info[..])?;
    client.p4info_mut().load(p4info);

    let p4name = "tna_p4rt_digest";
    let p4bin = include_bytes!("../build/tna_p4rt_digest/tofino/pipe/tofino.bin");
    let p4context = include_bytes!("../build/tna_p4rt_digest/tofino/pipe/context.json");
    let p4config = build_tofino_config(p4name, p4bin, p4context);

    println!("Setting pipeline config...");
    client.set_forwarding_pipeline_config(p4config).await?;

    let digest_entry = client.digest().new_entry("digest_a", 0, 1, 0);
    client.digest_mut().insert_entry(digest_entry).await?;

    loop {
        select! {
            _ = signal::ctrl_c() => {
                println!("Received Ctrl-C, shutting down...");
                break;
            },
            msg = client.get_digest(1) => {
                match msg {
                    Ok(digest) => {
                        let data = digest.data;
                        for d in data {
                            let d: Ipv4Digest = from_p4data(&d)?;
                            println!("Received digest: {:?}", d);
                        }
                    },
                    Err(e) => {
                    }
                }
            }
        }
    }

    Ok(())
}
