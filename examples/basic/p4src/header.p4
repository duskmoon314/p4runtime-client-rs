typedef bit<9> egress_port_t;

typedef bit<48> mac_addr_t;
typedef bit<32> ipv4_addr_t;
typedef bit<128> ipv6_addr_t;
typedef bit<16> vlan_id_t;

typedef bit<16> ether_type_t;
const ether_type_t ETHERTYPE_IPV4 = 16w0x0800;
const ether_type_t ETHERTYPE_ARP = 16w0x0806;
const ether_type_t ETHERTYPE_VLAN = 16w0x8100;
const ether_type_t ETHERTYPE_IPV6 = 16w0x86DD;

typedef bit<8> ip_protocol_t;
const ip_protocol_t IP_PROTO_TCP = 6;
const ip_protocol_t IP_PROTO_UDP = 17;

header ethernet_hdr_t {
    mac_addr_t dst;
    mac_addr_t src;
    ether_type_t ether_type;
}

header ipv4_hdr_t {
    bit<4> version;
    bit<4> ihl;
    bit<8> dscp;
    bit<16> total_len;
    bit<16> identification;
    bit<3> flags;
    bit<13> frag_offset;
    bit<8> ttl;
    ip_protocol_t protocol;
    bit<16> hdr_checksum;
    ipv4_addr_t src;
    ipv4_addr_t dst;
}

header ipv6_hdr_t {
    bit<4> version;
    bit<8> traffic_class;
    bit<20> flow_label;
    bit<16> payload_len;
    ip_protocol_t next_hdr;
    bit<8> hop_limit;
    ipv6_addr_t src;
    ipv6_addr_t dst;
}

header tcp_hdr_t {
    bit<16> src_port;
    bit<16> dst_port;
    bit<32> seq_no;
    bit<32> ack_no;
    bit<4> data_offset;
    bit<4> reserved;
    bit<8> flags;
    bit<16> window;
    bit<16> checksum;
    bit<16> urgent_ptr;
}

header udp_hdr_t {
    bit<16> src_port;
    bit<16> dst_port;
    bit<16> length;
    bit<16> checksum;
}

struct header_t {
    ethernet_hdr_t eth;
    ipv4_hdr_t ipv4;
    ipv6_hdr_t ipv6;
    tcp_hdr_t tcp;
    udp_hdr_t udp;
}

struct ipv4_digest_t {
    mac_addr_t dst_mac;
    ipv4_addr_t dst_ip;
    ip_protocol_t ip_proto;
}