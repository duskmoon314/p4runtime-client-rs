from mininet.net import Mininet
from mininet.topo import Topo
from mininet.log import setLogLevel
from mininet.cli import CLI

# Set path to load p4_mininet module
import sys

sys.path.append("../lib")

from p4_mininet import P4Switch, P4Host


def main():
    # Create network
    net = Mininet(
        switch=P4Switch,
        host=P4Host,
        controller=None,
    )

    # Add hosts and switches
    h1 = net.addHost("h1", ip="10.0.1.2/24", mac="08:00:00:00:01:02")
    h2 = net.addHost("h2", ip="10.0.2.2/24", mac="08:00:00:00:02:02")
    s1 = net.addSwitch(
        "s1",
        sw_path="simple_switch_grpc",
        grpc_port=9559,
    )
    # Add links
    net.addLink(h1, s1, 1)
    net.addLink(h2, s1, 2)

    net.start()

    # Setup routing on hosts
    h1 = net.get("h1")
    h1.setARP("10.0.1.1", "08:00:00:00:01:01")
    h1.setDefaultRoute("dev eth0 via 10.0.1.1")

    h2 = net.get("h2")
    h2.setARP("10.0.2.1", "08:00:00:00:02:01")
    h2.setDefaultRoute("dev eth0 via 10.0.2.1")

    # Start CLI
    CLI(net)

    # Stop network
    net.stop()


if __name__ == "__main__":
    setLogLevel("info")
    main()
