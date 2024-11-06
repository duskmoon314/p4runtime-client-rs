#include <core.p4>
#include <tna.p4>

#include "headers.p4"
#include "util.p4"

struct metadata_t {}

struct digest_a_t {
    mac_addr_t dst_mac;
    ipv4_addr_t dst_ip;
    bit<8> protocol;
}

parser SwitchIngressParser(
    packet_in pkt,
    out header_t hdr,
    out metadata_t ig_md,
    out ingress_intrinsic_metadata_t ig_intr_md) {

    TofinoIngressParser() tofino_parser;

    state start {
        tofino_parser.apply(pkt, ig_intr_md);
        transition parse_ethernet;
    }

    state parse_ethernet {
        pkt.extract(hdr.ethernet);
        transition select (hdr.ethernet.ether_type) {
            ETHERTYPE_IPV4 : parse_ipv4;
            default : reject;
        }
    }

    state parse_ipv4 {
        pkt.extract(hdr.ipv4);
        transition accept;
    }
}

control SwitchIngressDeparser(
        packet_out pkt,
        inout header_t hdr,
        in metadata_t ig_md,
        in ingress_intrinsic_metadata_for_deparser_t ig_intr_dprsr_md) {
    Digest<digest_a_t>() digest_a;

    apply {
        if (ig_intr_dprsr_md.digest_type == 1) {
            digest_a.pack({
                hdr.ethernet.dst_addr,
                hdr.ipv4.dst_addr,
                hdr.ipv4.protocol
            });
        }

        pkt.emit(hdr);
    }
}

control SwitchIngress(
        inout header_t hdr,
        inout metadata_t ig_md,
        in ingress_intrinsic_metadata_t ig_intr_md,
        in ingress_intrinsic_metadata_from_parser_t ig_intr_prsr_md,
        inout ingress_intrinsic_metadata_for_deparser_t ig_intr_dprsr_md,
        inout ingress_intrinsic_metadata_for_tm_t ig_intr_tm_md) {

    apply {
        // Digest all packets
        ig_intr_dprsr_md.digest_type = 1;

        // No need for egress
        ig_intr_tm_md.bypass_egress = 1w1;
    }
}

Pipeline(
    SwitchIngressParser(),
    SwitchIngress(),
    SwitchIngressDeparser(),
    EmptyEgressParser(),
    EmptyEgress(),
    EmptyEgressDeparser()) pipe;
Switch(pipe) main;