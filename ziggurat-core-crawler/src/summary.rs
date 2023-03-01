use std::{cmp, collections::HashMap, fmt, fs, net::SocketAddr, path::Path, time::Duration};

use serde::{Deserialize, Serialize};

// This struct contains a list of connection indices for each node
// It is equivalent to an adjacency or degree matrix, expressed in a compact form
type NodesIndices = Vec<Vec<usize>>;

/// Contains stats about crawled network.
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct NetworkSummary {
    /// Total number of nodes discovered.
    pub num_known_nodes: usize,
    /// Number of nodes that a crawler was able to connect to.
    pub num_good_nodes: usize,
    /// Total number of known connections as reported by peers.
    pub num_known_connections: usize,
    /// Number of all protocol versions discovered.
    pub num_versions: usize,
    /// Map: Version number -> number of nodes that reported this version.
    pub protocol_versions: HashMap<u32, usize>,
    /// Nodes' software versions.
    pub user_agents: HashMap<String, usize>,
    /// Crawler's runtime.
    pub crawler_runtime: Duration,
    /// Addresses of good nodes.
    pub node_addrs: Vec<SocketAddr>,
    /// Unidirected connections graph.
    pub nodes_indices: NodesIndices,
}

impl NetworkSummary {
    /// Logs current state of network to file.
    pub fn log_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        fs::write(path, self.to_string())?;
        Ok(())
    }
}

impl fmt::Display for NetworkSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_hashmap<T: fmt::Display>(
            f: &mut fmt::Formatter<'_>,
            counts: &HashMap<T, usize>,
        ) -> fmt::Result {
            let mut vec: Vec<(&T, &usize)> = counts.iter().collect();
            vec.sort_by_key(|(_, count)| cmp::Reverse(*count));

            for (item, count) in &vec {
                writeln!(f, "{item}: {count}")?;
            }

            Ok(())
        }

        writeln!(f, "Network summary:\n")?;
        writeln!(f, "Found a total of {} node(s)", self.num_known_nodes)?;
        writeln!(f, "Managed to connect to {} node(s)", self.num_good_nodes)?;
        writeln!(
            f,
            "{} identified themselves with a Version",
            self.num_versions
        )?;
        writeln!(
            f,
            "Nodes have {} known connections between them",
            self.num_known_connections
        )?;

        writeln!(f, "\nProtocol versions:")?;
        print_hashmap(f, &self.protocol_versions)?;
        writeln!(f, "\nUser agents:")?;
        print_hashmap(f, &self.user_agents)?;

        writeln!(
            f,
            "\nCrawler ran for a total of {} minutes",
            self.crawler_runtime.as_secs() / 60
        )?;

        Ok(())
    }
}
