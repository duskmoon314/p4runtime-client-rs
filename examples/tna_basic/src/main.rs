use p4runtime_client::client::Client;
use p4runtime_client::p4runtime::p4::v1 as p4_v1;
use p4runtime_client::p4runtime::p4::v1::Uint128;
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

    println!("Getting pipeline config...");
    let config = client
        .get_forwarding_pipeline_config(
            p4_v1::get_forwarding_pipeline_config_request::ResponseType::All,
        )
        .await?
        .into_inner();
    let p4info = config.config.unwrap().p4info.unwrap();
    client.p4info_mut().load(p4info);

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
