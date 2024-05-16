# Basic Example

This is a basic example of using `p4runtime-client` crate with `bmv2`.
Operations in this example include:

- Arbitration
- Setting pipeline config
- Adding table entries
- Receiving digest messages

## Prerequisites

- `Rust`
- `bmv2` (with `simple_switch_grpc`)
  - See [bmv2](https://github.com/p4lang/behavioral-model#installing-bmv2) for installation instructions
- `mininet` or `containerlab`
  - See [mininet](https://mininet.org/download/) or [containerlab](https://containerlab.dev/quickstart/)

## Running the example

### Using `mininet`

```shell
# This will start the mininet topology
$ sudo python basic.py
*** Configuring hosts
h1 h2 
*** Starting controller

*** Starting 1 switches
s1 Starting P4 switch s1simple_switch_grpc -i 1@s1-eth1 -i 2@s1-eth2 --nanolog ipc:///tmp/bm-0-log.ipc --device-id 0 --no-p4 --log-console -L trace -- --grpc-server-addr 0.0.0.0:9559
Switch has been started
*** Starting CLI:
mininet>
```

Then exec `h1 ping h2` in mininet CLI should not see any output since no table entries are added.

We can now run the rust code in another terminal:

```shell
$ cargo run --package basic
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/basic`
Server P4Runtime capabilities: "1.3.0"
Starting master arbitration...
Setting pipeline config...
Table entries inserted.
```

Now exec `h1 ping h2` in mininet CLI should see the ping packets are being sent.
And the rust code should print received digest messages:

```shell
Received message: StreamMessageResponse { update: Some(Digest(DigestList { digest_id: 386635259, list_id: 1, data: [P4Data { data: Some(Struct(P4StructLike { members: [P4Data { data: Some(Bitstring([8, 0, 0, 0, 2, 2])) }, P4Data { data: Some(Bitstring([10, 0, 2, 2])) }, P4Data { data: Some(Bitstring([1])) }] })) }], timestamp: 1528860281026415 })) }
Received message: StreamMessageResponse { update: Some(Digest(DigestList { digest_id: 386635259, list_id: 2, data: [P4Data { data: Some(Struct(P4StructLike { members: [P4Data { data: Some(Bitstring([8, 0, 0, 0, 1, 2])) }, P4Data { data: Some(Bitstring([10, 0, 1, 2])) }, P4Data { data: Some(Bitstring([1])) }] })) }], timestamp: 1528860283083110 })) }
Received message: StreamMessageResponse { update: Some(Digest(DigestList { digest_id: 386635259, list_id: 3, data: [P4Data { data: Some(Struct(P4StructLike { members: [P4Data { data: Some(Bitstring([8, 0, 0, 0, 2, 2])) }, P4Data { data: Some(Bitstring([10, 0, 2, 2])) }, P4Data { data: Some(Bitstring([1])) }] })) }], timestamp: 1528861282847311 })) }
```

### Using `containerlab`

Most of the steps are similar to the `mininet` example, except that we need to use `containerlab` to start the topology.

```shell
$ sudo clab deploy
INFO[0000] Containerlab v0.54.2 started                 
INFO[0000] Parsing & checking topology file: basic.clab.yml 
INFO[0000] Removing /path/to/p4runtime-client-rs/examples/basic/clab-basic directory... 
INFO[0000] Creating docker network: Name="clab", IPv4Subnet="172.20.20.0/24", IPv6Subnet="2001:172:20:20::/64", MTU=1500 
INFO[0000] Creating lab directory: /path/to/p4runtime-client-rs/examples/basic/clab-basic 
INFO[0000] Creating container: "r1"                     
INFO[0000] Creating container: "h1"                     
INFO[0000] Creating container: "h2"                     
INFO[0003] Created link: r1:eth1 <--> h1:eth1           
INFO[0003] Created link: r1:eth2 <--> h2:eth1           
INFO[0005] Executed command "ip addr add 10.0.2.2/24 dev eth1" on the node "h2". stdout: 
INFO[0005] Executed command "ip neigh add 10.0.2.1 lladdr 08:00:00:00:02:01 dev eth1" on the node "h2". stdout: 
INFO[0005] Executed command "ip route change default via 10.0.2.1 dev eth1" on the node "h2". stdout: 
INFO[0005] Executed command "bash -c simple_switch_grpc -i 1@eth1 -i 2@eth2 --no-p4 --log-file /var/log/simple_switch/log --log-flush -L trace &" on the node "r1". stdout:
Calling target program-options parser
Adding interface eth1 as port 1
Adding interface eth2 as port 2 
INFO[0005] Executed command "ip addr add 10.0.1.2/24 dev eth1" on the node "h1". stdout: 
INFO[0005] Executed command "ip neigh add 10.0.1.1 lladdr 08:00:00:00:01:01 dev eth1" on the node "h1". stdout: 
INFO[0005] Executed command "ip route change default via 10.0.1.1 dev eth1" on the node "h1". stdout: 
INFO[0005] Adding containerlab host entries to /etc/hosts file 
INFO[0005] Adding ssh config for containerlab nodes     
+---+---------------+--------------+--------------------------------+-------+---------+----------------+----------------------+
| # |     Name      | Container ID |             Image              | Kind  |  State  |  IPv4 Address  |     IPv6 Address     |
+---+---------------+--------------+--------------------------------+-------+---------+----------------+----------------------+
| 1 | clab-basic-h1 | 7e1bb67322ff | duskmoon/dev-env:bs-u22        | linux | running | 172.20.20.2/24 | 2001:172:20:20::2/64 |
| 2 | clab-basic-h2 | 302e5af1ac62 | duskmoon/dev-env:bs-u22        | linux | running | 172.20.20.4/24 | 2001:172:20:20::4/64 |
| 3 | clab-basic-r1 | 1603f2d41564 | p4lang/behavioral-model:latest | linux | running | 172.20.20.3/24 | 2001:172:20:20::3/64 |
+---+---------------+--------------+--------------------------------+-------+---------+----------------+----------------------+
```

After starting the rust code, we can exec `docker exec -it clab-basic-h1 ping 10.0.2.2` to see the ping packets are being sent:

```shell
$ docker exec -it clab-basic-h1 ping 10.0.2.2
PING 10.0.2.2 (10.0.2.2) 56(84) bytes of data.
64 bytes from 10.0.2.2: icmp_seq=1 ttl=63 time=3394 ms
64 bytes from 10.0.2.2: icmp_seq=2 ttl=63 time=2376 ms
64 bytes from 10.0.2.2: icmp_seq=3 ttl=63 time=1353 ms
64 bytes from 10.0.2.2: icmp_seq=4 ttl=63 time=329 ms
64 bytes from 10.0.2.2: icmp_seq=5 ttl=63 time=1.29 ms
64 bytes from 10.0.2.2: icmp_seq=6 ttl=63 time=1.36 ms
64 bytes from 10.0.2.2: icmp_seq=7 ttl=63 time=1.12 ms
64 bytes from 10.0.2.2: icmp_seq=8 ttl=63 time=1.30 ms
^C
--- 10.0.2.2 ping statistics ---
8 packets transmitted, 8 received, 0% packet loss, time 7070ms
rtt min/avg/max/mdev = 1.121/932.092/3393.901/1232.534 ms, pipe 4
```