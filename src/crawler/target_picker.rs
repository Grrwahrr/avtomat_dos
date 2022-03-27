use crate::targets::{hash_target, Target};
use std::collections::HashMap;
use std::ops::Add;
use std::time::{Duration, SystemTime};

/// The state of a specific target
#[derive(PartialEq)]
pub enum TargetState {
    Unknown,
    Online,
    Offline,
}

/// Info about the target, whether it is online and when we can next send requests
struct TargetInfo {
    target: Target,
    state: TargetState,
    blocked_until: SystemTime,
}

/// Data for picking the next target
pub struct TargetPicker {
    /// A map holding all known targets
    targets: HashMap<u64, TargetInfo>,

    /// An offset for finding the next target in the map
    offset: usize,

    /// Last time the target list was updated
    next_update: SystemTime,
}

impl TargetPicker {
    /// How often to update targets
    const UPDATE_TIME_IN_SECS: u64 = 7200; // 7200 = 120 minutes

    /// How long to block a target for when starting a request (assume its offline)
    const DEFAULT_BLOCK_TIME: u64 = 300; // Make this a sensible time = 300 = 5 minutes

    /// How long to block an online target for.
    /// A presumably safe time of 3 seconds - we do not want to get blocked.
    const DEFAULT_ONLINE_BLOCK_TIME: u64 = 3;

    /// Constructor
    pub fn new() -> Self {
        Self {
            targets: HashMap::new(),
            next_update: SystemTime::now(),
            offset: 0,
        }
    }

    /// Return the next viable target. Or None if nothing was found
    pub fn block_next_target(&mut self) -> Option<Target> {
        if self.targets.is_empty() {
            return None;
        }

        // The current time to check against
        let now = SystemTime::now();

        // Go through targets starting at offset
        for target_info in self.targets.values_mut().skip(self.offset) {
            self.offset += 1;

            if now > target_info.blocked_until {
                // Update the target blocked until time as if it were offline
                target_info.blocked_until =
                    now.add(Duration::from_secs(TargetPicker::DEFAULT_BLOCK_TIME));

                // Return the target
                return Some(target_info.target.clone());
            }
        }

        self.offset = 0;

        None
    }

    /// Should the target list be updated. Returns true if it's time to update
    pub fn start_update_targets(&mut self) -> bool {
        if SystemTime::now() > self.next_update {
            // Set next update time
            self.next_update =
                SystemTime::now().add(Duration::from_secs(TargetPicker::UPDATE_TIME_IN_SECS));
            return true;
        }

        false
    }

    /// Add the given targets to our target map. Returns the total number of targets
    pub fn add_targets(&mut self, targets: Vec<Target>) -> Vec<String> {
        let mut all_targets = vec![];

        for target in targets {
            let itm = self
                .targets
                .entry(hash_target(&target))
                .or_insert(TargetInfo {
                    target,
                    state: TargetState::Unknown,
                    blocked_until: SystemTime::now(),
                });

            all_targets.push(itm.target.to_string());
        }

        all_targets
    }

    /// Change the state of some target
    pub fn report_target_status(&mut self, target: Target, state: TargetState) -> (i32, i32) {
        let (mut online, mut offline) = (0, 0);

        match self.targets.get_mut(&hash_target(&target)) {
            None => {}
            Some(e) => {
                if e.state == TargetState::Unknown {
                    if state == TargetState::Online {
                        online = 1;
                    } else if state == TargetState::Offline {
                        offline = 1;
                    }
                } else if e.state == TargetState::Online && state == TargetState::Offline {
                    online = -1;
                    offline = 1;
                } else if e.state == TargetState::Offline && state == TargetState::Online {
                    offline = -1;
                    online = 1;
                }

                // Modify blocked_until if the target is online
                if state == TargetState::Online {
                    e.blocked_until = SystemTime::now()
                        .add(Duration::from_secs(TargetPicker::DEFAULT_ONLINE_BLOCK_TIME));
                }

                // Modify state
                e.state = state;
            }
        }

        (online, offline)
    }
}
