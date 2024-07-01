# TNA Basic Example

This is a basic example of using `p4runtime-client` crate with `Tofino`.

Operations in this example include:

- Arbitration
- Setting pipeline config
- Reading counter entries

> I tested this example with SDE 9.7.5, which only supports P4Runtime v1.0.0. That is, it does not support `Capabilities` and some other operations that are available in later versions of P4Runtime.

## Prerequisites

- `Rust`
- SDE 9.7.5
  - Other versions may work, but I don't have enough resources and time to test them.

## Running the example

1. Make sure all the prerequisites are installed.
2. Set up the environment variables for the SDE.
   - `SDE` and `SDE_INSTALL` are required.
   - `SDE` should be something like `/path/to/bf-sde-9.7.5`.
3. Build the P4 program by `make build`.
4. Start the `bf_switchd` by `make start` in one terminal.
5. Start the control plane by `cargo run --package tna_basic` in another terminal.
6. Setup port and multicast group by `ucli`, so that the data plane can receive packets.
   - TODO: add more guidance here.

The expected output would be like:

<details>
<summary> Terminal 1: `make start` </summary>

```console
$ make start
/home/dev/bf-sde-9.7.5/run_switchd.sh -p tna_p4rt_basic --p4rt-server 0.0.0.0:9559
Using SDE /home/dev/bf-sde-9.7.5
Using SDE_INSTALL /home/dev/bf-sde-9.7.5/install
Setting up DMA Memory Pool
[sudo] password for dev: 
Using TARGET_CONFIG_FILE /home/dev/bf-sde-9.7.5/install/share/p4/targets/tofino/tna_p4rt_basic.conf
Using PATH /home/dev/bf-sde-9.7.5/install/bin:/home/dev/.cargo/bin:/usr/local/bin:/usr/bin:/bin
Using LD_LIBRARY_PATH /usr/local/lib:/home/dev/bf-sde-9.7.5/install/lib:
bf_sysfs_fname /sys/class/bf/bf0/device/dev_add
Install dir: /home/dev/bf-sde-9.7.5/install (0x55e16e626960)
bf_switchd: system services initialized
bf_switchd: loading conf_file /home/dev/bf-sde-9.7.5/install/share/p4/targets/tofino/tna_p4rt_basic.conf...
bf_switchd: processing device configuration...
Configuration for dev_id 0
  Family        : tofino
  pci_sysfs_str : /sys/devices/pci0000:00/0000:00:03.0/0000:05:00.0
  pci_domain    : 0
  pci_bus       : 0
  pci_fn        : 0
  pci_dev       : 0
  pci_int_mode  : 0
  sbus_master_fw: /home/dev/bf-sde-9.7.5/install/
  pcie_fw       : /home/dev/bf-sde-9.7.5/install/
  serdes_fw     : /home/dev/bf-sde-9.7.5/install/
  sds_fw_path   : /home/dev/bf-sde-9.7.5/install/share/tofino_sds_fw/avago/firmware
  microp_fw_path: 
bf_switchd: processing P4 configuration...
P4 profile for dev_id 0
num P4 programs 1
  p4_name: tna_p4rt_basic
  p4_pipeline_name: pipe
    libpd: 
    libpdthrift: 
    context: /home/dev/bf-sde-9.7.5/install/share/tofinopd/tna_p4rt_basic/pipe/context.json
    config: /home/dev/bf-sde-9.7.5/install/share/tofinopd/tna_p4rt_basic/pipe/tofino.bin
  Pipes in scope [0 1 2 3 ]
  diag: 
  accton diag: 
  Agent[0]: /home/dev/bf-sde-9.7.5/install/lib/libpltfm_mgr.so
  non_default_port_ppgs: 0
  SAI default initialize: 1 
bf_switchd: library /home/dev/bf-sde-9.7.5/install/lib/libpltfm_mgr.so loaded
bf_switchd: agent[0] initialized
Health monitor started 
Operational mode set to ASIC
Initialized the device types using platforms infra API
ASIC detected at PCI /sys/class/bf/bf0/device
ASIC pci device id is 16
Starting PD-API RPC server on port 9090
bf_switchd: drivers initialized
Setting core_pll_ctrl0=cd44cbfe
-
bf_switchd: dev_id 0 initialized

bf_switchd: initialized 1 devices
Adding Thrift service for bf-platforms to server
bf_switchd: thrift initialized for agent : 0
bf_switchd: spawning cli server thread
bf_switchd: spawning driver shell
bf_switchd: server started - listening on port 9999
bfruntime gRPC server started on 0.0.0.0:50052
Server listening on 0.0.0.0:9559
P4Runtime GRPC server started on 0.0.0.0:9559

        ********************************************
        *      WARNING: Authorised Access Only     *
        ********************************************
    

bfshell> bf_switchd: starting warm init for dev_id 0 mode 1 serdes_upgrade 0 
Removing Thrift service for bf-platforms from server
bf_switchd: thrift deinitialized for agent : 0
bf_switchd: agent[0] library unloaded for dev_id 0
bf_switchd: library /home/dev/bf-sde-9.7.5/install/lib/libpltfm_mgr.so loaded
bf_switchd: agent[0] initialized
Health monitor started 
Adding Thrift service for bf-platforms to server
bf_switchd: thrift initialized for agent : 0
\2024-07-01 06:44:19.774069 BF_BFRT ERROR - bfRtDeviceAdd:349 No BF-RT json file found for program tna_p4rt_basic Not adding BF-RT Info object for it
Starting UCLI from bf-shell 
Starting UCLI from bf-shell
```

</details>

> Notes:
>
> The two `Starting UCLI from bf-shell` lines indicate that ports and multicast groups are set up by `ucli`.

<details>
<summary> Terminal 2: `cargo run --package tna_basic` </summary>

```console
$ cargo run --package tna_basic
   Compiling tna_basic v0.1.0 (/home/dev/projects/p4runtime-client-rs/examples/tna_basic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.77s
     Running `target/debug/tna_basic`
Starting master arbitration...
Setting pipeline config...
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 0, packet_count: 0 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 441, packet_count: 2 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 645, packet_count: 4 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 1113, packet_count: 8 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 2286, packet_count: 15 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 2751, packet_count: 20 }) }]
Counter entries: [CounterEntry { counter_id: 313692501, index: Some(Index { index: 0 }), data: Some(CounterData { byte_count: 3768, packet_count: 26 }) }]
```

> Notes:
>
> The counter entries are read every second. Zero values are expected at the beginning because ports and multicast groups are not set up yet. The values increase as the data plane receives packets.

</details>

## License

The `header.p4` and `util.p4` files are from the examples provided with the SDE. They are licensed as their comments say.