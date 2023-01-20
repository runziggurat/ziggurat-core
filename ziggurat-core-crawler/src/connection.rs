use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    net::SocketAddr,
    time::Instant,
};

/// A connection found in the network.
#[derive(Debug, Eq, Copy, Clone)]
pub struct KnownConnection {
    /// One of the two sides of a connection.
    pub a: SocketAddr,
    /// The other side of a connection.
    pub b: SocketAddr,
    /// The timestamp of the last time the connection was seen.
    pub last_seen: Instant,
}

impl Hash for KnownConnection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (a, b) = (self.a, self.b);

        // This ensures the hash is the same for (a, b) as it is for (b, a).
        match a.cmp(&b) {
            Ordering::Greater => {
                b.hash(state);
                a.hash(state);
            }
            _ => {
                a.hash(state);
                b.hash(state);
            }
        }
    }
}

impl KnownConnection {
    pub fn new(a: SocketAddr, b: SocketAddr) -> Self {
        Self {
            a,
            b,
            last_seen: Instant::now(),
        }
    }
}

impl PartialEq for KnownConnection {
    fn eq(&self, other: &Self) -> bool {
        let (a, b) = (self.a, self.b);
        let (c, d) = (other.a, other.b);

        a == d && b == c || a == c && b == d
    }
}
