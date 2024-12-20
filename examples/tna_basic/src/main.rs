use p4runtime_client::client::Client;
use p4runtime_client::config::build_tofino_config;
use p4runtime_client::p4runtime::p4::config::v1 as p4_cfg_v1;
use p4runtime_client::p4runtime::p4::v1::Uint128;
use prost::Message;
use tokio::{select, signal, time::sleep, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let p4name = "tna_p4rt_basic";
    let p4bin = include_bytes!("../build/tna_p4rt_basic/tofino/pipe/tofino.bin");
    let p4context = include_bytes!("../build/tna_p4rt_basic/tofino/pipe/context.json");
    let p4config = build_tofino_config(p4name, p4bin, p4context);

    println!("Setting pipeline config...");
    client.set_forwarding_pipeline_config(p4config).await?;

    let counter_entry = client.counter().new_entry("counter", None, None);

    loop {
        select! {
            _ = sleep(Duration::from_secs(1)) => {
                let entries = client.counter_mut().read_entries(counter_entry).await?;

                println!("Counter entries: {:?}", entries);
            },

            _ = signal::ctrl_c() => {
                println!("Ctrl-C received, shutting down...");
                break;
            }
        }
    }

    Ok(())
}
