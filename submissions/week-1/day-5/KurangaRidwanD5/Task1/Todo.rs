use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::{Duration, Instant};

// Mock types to simulate the original code
#[derive(Debug, Clone, PartialEq)]
enum ConnectionStatus {
    Connected { multiaddr: Multiaddr, direction: ConnectionDirection },
    Disconnected { instant: Instant },
    Banned { instant: Instant },
}

#[derive(Debug, Clone, PartialEq)]
enum ConnectionDirection {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone, PartialEq)]
struct Multiaddr {
    ip: IpAddr,
}

impl Multiaddr {
    fn new(ip: &str) -> Self {
        Multiaddr {
            ip: IpAddr::from_str(ip).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Reputation {
    Neutral,
    Banned,
}

#[derive(Debug)]
struct Peer {
    status: ConnectionStatus,
    ip_addresses: Vec<IpAddr>,
    reputation: Reputation,
}

impl Peer {
    fn new(ip: IpAddr) -> Self {
        Peer {
            status: ConnectionStatus::Disconnected { instant: Instant::now() },
            ip_addresses: vec![ip],
            reputation: Reputation::Neutral,
        }
    }

    fn connection_status(&self) -> &ConnectionStatus {
        &self.status
    }

    fn set_connection_status(&mut self, status: ConnectionStatus) {
        self.status = status;
    }

    fn known_ip_addresses(&self) -> impl Iterator<Item = &IpAddr> {
        self.ip_addresses.iter()
    }

    fn register_incoming(&mut self, multiaddr: Multiaddr) {
        self.ip_addresses.push(multiaddr.ip);
        self.status = ConnectionStatus::Connected {
            multiaddr,
            direction: ConnectionDirection::Incoming,
        };
    }

    fn reputation(&self) -> Reputation {
        self.reputation.clone()
    }

    fn set_reputation(&mut self, reputation: Reputation) {
        self.reputation = reputation;
    }
}

#[derive(Debug)]
struct BannedPeers {
    banned_ips: HashSet<IpAddr>,
}

impl BannedPeers {
    fn new() -> Self {
        BannedPeers {
            banned_ips: HashSet::new(),
        }
    }

    fn add_banned_peer(&mut self, peer: &Peer) {
        for ip in peer.known_ip_addresses() {
            self.banned_ips.insert(*ip);
        }
    }

    fn banned_ips(&self) -> &HashSet<IpAddr> {
        &self.banned_ips
    }

    fn ip_banned(&self, ip: &IpAddr) -> bool {
        self.banned_ips.contains(ip)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PeerId(String);

#[derive(Debug)]
struct AllPeers {
    peers: HashMap<PeerId, Peer>,
    banned_peers: BannedPeers,
    disconnected_peers: usize,
}

impl AllPeers {
    fn new() -> Self {
        AllPeers {
            peers: HashMap::new(),
            banned_peers: BannedPeers::new(),
            disconnected_peers: 0,
        }
    }

    fn handle_disconnected_and_banned(&mut self, peer_id: &PeerId) {
        let already_banned_ips = self.banned_peers.banned_ips();
        if let Some(peer) = self.peers.get_mut(peer_id) {
            peer.set_connection_status(ConnectionStatus::Banned { instant: Instant::now() });
            peer.set_reputation(Reputation::Banned);
            self.banned_peers.add_banned_peer(peer);
            let banned_ips = peer
                .known_ip_addresses()
                .filter(|ip| already_banned_ips.contains(ip))
                .cloned()
                .collect::<Vec<_>>();
            println!("Banned IPs: {:?}", banned_ips);
        } else {
            println!("Peer not found: {:?}", peer_id);
        }
    }

    fn peer_banned(&self, peer_id: &PeerId) -> bool {
        self.peers.get(peer_id).map_or(false, |peer| {
            peer.reputation() == Reputation::Banned
                || peer.known_ip_addresses().any(|ip| self.banned_peers.ip_banned(ip))
        })
    }

    fn handle_connect_attempt(&mut self, peer_id: PeerId, new_ip: &str) -> bool {
        let multiaddr = Multiaddr::new(new_ip);
        if self.peer_banned(&peer_id) {
            println!("Connection rejected for banned peer: {:?}", peer_id);
            return false;
        }
        if let Some(peer) = self.peers.get_mut(&peer_id) {
            peer.register_incoming(multiaddr);
            println!("Connection accepted for peer: {:?}", peer_id);
            return true;
        }
        // Simulate new peer connection
        let mut new_peer = Peer::new(IpAddr::from_str(new_ip).unwrap());
        new_peer.register_incoming(multiaddr);
        self.peers.insert(peer_id.clone(), new_peer);
        println!("Connection accepted for new peer: {:?}", peer_id);
        true
    }
}

fn main() {
    let mut all_peers = AllPeers::new();
    
    // Step 1: Add a peer with IP "192.168.1.1"
    let peer_id = PeerId("P1".to_string());
    let initial_ip = IpAddr::from_str("192.168.1.1").unwrap();
    all_peers.peers.insert(peer_id.clone(), Peer::new(initial_ip));
    
    // Step 2: Ban the peer
    println!("Banning peer {:?}", peer_id);
    all_peers.handle_disconnected_and_banned(&peer_id);
    
    // Step 3: Attempt to reconnect with the same IP (should fail)
    let connect_same_ip = all_peers.handle_connect_attempt(peer_id.clone(), "192.168.1.1");
    assert!(!connect_same_ip, "Connection with banned IP should fail");
    
    // Step 4: Attempt to reconnect with a new IP (vulnerability: should succeed)
    let connect_new_ip = all_peers.handle_connect_attempt(peer_id.clone(), "192.168.1.2");
    assert!(connect_new_ip, "Vulnerability: Connection with new IP succeeds");
    
    // Verify peer status
    if let Some(peer) = all_peers.peers.get(&peer_id) {
        println!("Peer status after reconnect: {:?}", peer.connection_status());
        println!("Peer IPs: {:?}", peer.known_ip_addresses().collect::<Vec<_>>());
    }
}