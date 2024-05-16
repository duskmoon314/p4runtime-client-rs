#include <core.p4>
#include <v1model.p4>

#include "header.p4"

struct metadata {}

parser SwitchParser(
    packet_in packet,
    out header_t hdr,
    inout metadata meta,
    inout standard_metadata_t standard_metadata
) {
    state start {
        transition parse_ethernet;
    }

    state parse_ethernet {
        packet.extract(hdr.eth);
        transition select(hdr.eth.ether_type) {
            ETHERTYPE_IPV4: parse_ipv4;
            ETHERTYPE_IPV6: parse_ipv6;
            default: accept;
        }
    }

    state parse_ipv4 {
        packet.extract(hdr.ipv4);
        transition accept;
    }

    state parse_ipv6 {
        packet.extract(hdr.ipv6);
        transition accept;
    }
}

control SwitchVerifyChecksum(
    inout header_t hdr,
    inout metadata meta
) {
    apply { }
}

control SwitchIngress(
    inout header_t hdr,
    inout metadata meta,
    inout standard_metadata_t standard_metadata
) {
    action drop() {
        mark_to_drop(standard_metadata);
    }

    action ipv4_forward(mac_addr_t dst, egress_port_t port) {
        standard_metadata.egress_spec = port;
        hdr.eth.src = hdr.eth.dst;
        hdr.eth.dst = dst;
        hdr.ipv4.ttl = hdr.ipv4.ttl - 1;

        digest<ipv4_digest_t>(1, {
            hdr.eth.dst,
            hdr.ipv4.dst,
            hdr.ipv4.protocol
        });
    }

    action ipv6_forward(mac_addr_t dst, egress_port_t port) {
        standard_metadata.egress_spec = port;
        hdr.eth.src = hdr.eth.dst;
        hdr.eth.dst = dst;
        hdr.ipv6.hop_limit = hdr.ipv6.hop_limit - 1;
    }

    table ipv4_lpm {
        key = {
            hdr.ipv4.dst: lpm;
        }
        actions = {
            ipv4_forward;
            drop;
        }
        size = 1024;
        default_action = drop();
    }

    table ipv6_lpm {
        key = {
            hdr.ipv6.dst: lpm;
        }
        actions = {
            ipv6_forward;
            drop;
        }
        size = 1024;
        default_action = drop();
    }

    apply {
        if (hdr.ipv4.isValid()) {
            ipv4_lpm.apply();
        } else if (hdr.ipv6.isValid()) {
            ipv6_lpm.apply();
        }
    }
}

control SwitchEgress(
    inout header_t hdr,
    inout metadata meta,
    inout standard_metadata_t standard_metadata
) {
    apply { }
}

control SwitchComputeChecksum(
    inout header_t hdr,
    inout metadata meta
) {
    apply { 
        update_checksum(
            hdr.ipv4.isValid(),
            {
                hdr.ipv4.version,
                hdr.ipv4.ihl,
                hdr.ipv4.dscp,
                hdr.ipv4.total_len,
                hdr.ipv4.identification,
                hdr.ipv4.flags,
                hdr.ipv4.frag_offset,
                hdr.ipv4.ttl,
                hdr.ipv4.protocol,
                hdr.ipv4.src,
                hdr.ipv4.dst
            },
            hdr.ipv4.hdr_checksum,
            HashAlgorithm.csum16
        );
    }
}

control SwitchDeparser(
    packet_out packet,
    in header_t hdr
) {
    apply {
        packet.emit(hdr.eth);
        packet.emit(hdr.ipv4);
        packet.emit(hdr.ipv6);
    }
}

V1Switch(
    SwitchParser(),
    SwitchVerifyChecksum(),
    SwitchIngress(),
    SwitchEgress(),
    SwitchComputeChecksum(),
    SwitchDeparser()
) main;
