use crate::crawler::header_db::{HeaderDB, Headers};
use crate::crawler::target_picker::{TargetPicker, TargetState};
use crate::targets::{fetch_targets, Target};
use rand::{seq::SliceRandom, thread_rng};
use reqwest::{header, Request};
use select::document::Document;
use select::predicate::Name;
use std::default::Default;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::{mpsc, mpsc::Receiver};
use tokio::time::sleep;
use url::Url;

mod header_db;
mod target_picker;

/// Indicates whether the crawler is running or not
#[derive(PartialEq)]
pub enum CrawlerState {
    Running,
    Stopped,
}

/// A bunch of stats the crawler keeps track of
#[derive(Default, Clone)]
pub struct CrawlerStats {
    pub targets: u32,
    pub online: u32,
    pub offline: u32,
    pub requests: u64,
}

/// A report send back to the command task indicating the result
enum CrawlerTaskMessage {
    TaskFinished {
        target: Target,
        state: TargetState,
        requests_send: u32,
    },
    TargetsAcquired {
        targets: Vec<Target>,
    },
}

/// The crawler
/// In accordance with the configured intensity the crawler will spawn additional tasks that
/// will send requests to the given target and send a report back on completion.
pub struct Crawler {
    stats: Arc<RwLock<CrawlerStats>>,
    intensity: Arc<RwLock<u8>>,
    state: Arc<RwLock<CrawlerState>>,
    targets: Arc<RwLock<Vec<String>>>,
}

impl Crawler {
    pub fn new() -> Self {
        let crawler = Self {
            stats: Arc::new(RwLock::new(CrawlerStats::default())),
            intensity: Arc::new(RwLock::new(50)),
            state: Arc::new(RwLock::new(CrawlerState::Stopped)),
            targets: Arc::new(RwLock::new(Vec::new())),
        };

        let stats = crawler.stats.clone();
        let intensity = crawler.intensity.clone();
        let state = crawler.state.clone();
        let targets = crawler.targets.clone();
        tokio::spawn(async move {
            // Create a channel for tasks to report back on
            let (tx, mut rx) = mpsc::channel::<CrawlerTaskMessage>(100);

            // Info we need to pick the next target
            let mut picker = TargetPicker::new();

            // The number of active crawler tasks
            let mut active_tasks = 0u32;

            // Headers database
            let headers = HeaderDB::new();

            loop {
                // 1) Are we running?
                if !crawler_can_run(&state) {
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }

                // 2) Check for messages on the report channel
                crawler_process_messages(
                    &mut rx,
                    &mut active_tasks,
                    &mut picker,
                    stats.clone(),
                    targets.clone(),
                );

                // 3) Update the target list, will check if update is required
                crawler_update_targets(&mut picker, tx.clone());

                // 4) Can we spawn more tasks?
                if let Some(target) =
                    crawler_can_spawn_task_for_target(active_tasks, &intensity, &mut picker)
                {
                    active_tasks += 1;
                    crawler_fetch_target(target, headers.get_random_headers(), tx.clone());
                } else {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        });

        crawler
    }

    /// Update the intensity - that means the number of tasks sending requests in parallel
    pub fn set_intensity(&self, intensity: u8) -> bool {
        match self.intensity.write() {
            Ok(mut guard) => {
                *guard = intensity;
                true
            }
            _ => false,
        }
    }

    /// Start sending requests
    pub fn start(&self) -> bool {
        match self.state.write() {
            Ok(mut guard) => {
                *guard = CrawlerState::Running;
                true
            }
            _ => false,
        }
    }

    /// Stop sending requests
    pub fn stop(&self) -> bool {
        match self.state.write() {
            Ok(mut guard) => {
                *guard = CrawlerState::Stopped;
                true
            }
            _ => false,
        }
    }

    /// Get current statistics
    pub fn get_stats(&self) -> CrawlerStats {
        match self.stats.read() {
            Ok(guard) => guard.clone(),
            _ => Default::default(),
        }
    }

    /// Get current targets
    pub fn get_targets(&self) -> Vec<String> {
        match self.targets.read() {
            Ok(guard) => guard.clone(),
            _ => Default::default(),
        }
    }
}

/// Helper function that checks if we are in a running state
fn crawler_can_run(state: &Arc<RwLock<CrawlerState>>) -> bool {
    match state.read() {
        Ok(guard) => *guard == CrawlerState::Running,
        Err(_) => false,
    }
}

/// Helper function that will determine, based on the current intensity, if more tasks can be spawned
fn crawler_can_spawn_task_for_target(
    active_tasks: u32,
    intensity: &Arc<RwLock<u8>>,
    picker: &mut TargetPicker,
) -> Option<Target> {
    if let Ok(guard) = intensity.read() {
        if active_tasks > (*guard as u32 * 2) {
            return None;
        }
    };

    picker.block_next_target()
}

/// Helper function to loop over available messages and send them to the processor
fn crawler_process_messages(
    rx: &mut Receiver<CrawlerTaskMessage>,
    active_tasks: &mut u32,
    picker: &mut TargetPicker,
    stats: Arc<RwLock<CrawlerStats>>,
    targets: Arc<RwLock<Vec<String>>>,
) {
    while let Ok(msg) = rx.try_recv() {
        crawler_process_task_report(msg, active_tasks, picker, stats.clone(), targets.clone());
    }
}

/// Helper function to process the reply of some task
fn crawler_process_task_report(
    msg: CrawlerTaskMessage,
    active_tasks: &mut u32,
    picker: &mut TargetPicker,
    stats: Arc<RwLock<CrawlerStats>>,
    targets_ui: Arc<RwLock<Vec<String>>>,
) {
    match msg {
        CrawlerTaskMessage::TaskFinished {
            target,
            state,
            requests_send,
        } => {
            println!("TaskFinished: {target:?} {requests_send}");

            // Reduce the number of active tasks
            *active_tasks -= 1;

            // Update the target picker
            let (mod_online, mod_offline) = picker.report_target_status(target, state);

            // Update statistics
            if let Ok(mut guard) = stats.write() {
                guard.online = (guard.online as i32 + mod_online) as u32;
                guard.offline = (guard.offline as i32 + mod_offline) as u32;
                guard.requests += requests_send as u64;
            }
        }
        CrawlerTaskMessage::TargetsAcquired { targets } => {
            let targets = picker.add_targets(targets);

            if let Ok(mut guard) = stats.write() {
                guard.targets = targets.len() as u32;
            }

            if let Ok(mut guard) = targets_ui.write() {
                *guard = targets;
            }
        }
    }
}

/// Fetch the target list in a separate task and send a CrawlerTaskMessage::TargetsAcquired on completion
fn crawler_update_targets(picker: &mut TargetPicker, tx: mpsc::Sender<CrawlerTaskMessage>) {
    if !picker.start_update_targets() {
        return;
    }

    tokio::spawn(async move {
        println!("Checking for updated target list...");

        // Try to get the updated target list and randomize the order of targets
        let targets = match fetch_targets().await {
            Some(mut vec) => {
                vec.shuffle(&mut thread_rng());
                vec
            }
            _ => {
                return;
            }
        };

        // Send the targets back to the control task for processing
        if let Err(e) = tx
            .send(CrawlerTaskMessage::TargetsAcquired { targets })
            .await
        {
            println!("Could not send CrawlerTaskMessage::TargetsAcquired to control thread. {e}");
        };
    });
}

/// Request some resource
fn crawler_fetch_target(target: Target, headers: Headers, tx: mpsc::Sender<CrawlerTaskMessage>) {
    tokio::spawn(async move {
        let (requests_send, state) = match &target {
            Target::Https { url } => {
                match crawler_crawl_website(format!("https://{url}"), headers).await {
                    Some(tmp) => tmp,
                    _ => (0, TargetState::Unknown),
                }
            }
            Target::Http { url } => {
                match crawler_crawl_website(format!("http://{url}"), headers).await {
                    Some(tmp) => tmp,
                    _ => (0, TargetState::Unknown),
                }
            }
            Target::Udp { ip, port } => {
                todo!("Udp {ip} {port}");
                //(0, TargetState::Unknown)
            }
            Target::Tcp { ip, port } => {
                todo!("Tcp {ip} {port}");
                //(0, TargetState::Unknown)
            }
        };

        if let Err(e) = tx
            .send(CrawlerTaskMessage::TaskFinished {
                target,
                state,
                requests_send,
            })
            .await
        {
            println!("Could not send CrawlerTaskMessage::TaskFinished to control thread. {e}");
        };
    });
}

async fn crawler_crawl_website(url: String, headers: Headers) -> Option<(u32, TargetState)> {
    // Build the headers
    let mut header_map = header::HeaderMap::new();
    header_map.insert(
        "Accept",
        header::HeaderValue::from_bytes(headers.accept.as_bytes()).ok()?,
    );
    header_map.insert(
        "Accept-Language",
        header::HeaderValue::from_bytes(headers.accept_language.as_bytes()).ok()?,
    );
    // Do not request encoded replies yet, bigger HTML is good + no need to use decoders
    //header_map.insert("Accept-Encoding", header::HeaderValue::from_bytes(&headers.accept_encoding.as_bytes()).ok()?);

    // Build the client
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .user_agent(headers.agent)
        .default_headers(header_map)
        .tcp_keepalive(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .build()
        .ok()?;

    // Count send requests
    let mut requests_send = 1;
    let mut target_state = TargetState::Offline;

    // Figure the host, doctor it a little
    let host = Url::parse(&url).ok()?.host_str()?.replace("www.", "");

    // Build the initial request
    if let Ok(res) = client.get(&url).send().await {
        target_state = TargetState::Online;

        // Get the content
        let html = res.text().await.unwrap_or_default();

        // Try to find pictures on the same host, build a get request for each
        let image_requests: Vec<Request> = Document::from(html.as_str())
            .find(Name("img"))
            .filter_map(|node| node.attr("src"))
            .filter(|src| src.contains(&host) || !src.contains("://"))
            .filter_map(|img| client.get(img).build().ok())
            .collect();

        // Create a bunch of futures to fetch images
        let futures = image_requests
            .into_iter()
            .map(|img_req| client.execute(img_req));

        // Send requests and wait for all images
        let responses = futures::future::join_all(futures).await;

        // // Create futures to fetch contents
        // let futures = responses
        //     .into_iter()
        //     .filter_map(|res| res.ok())
        //     .map(|res| res.bytes());
        //
        // // Fetch all the data
        // let responses = futures::future::join_all(futures).await;

        // Update the request counter
        requests_send += responses.len() as u32;
    }

    Some((requests_send, target_state))
}
