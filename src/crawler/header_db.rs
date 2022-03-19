use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Headers {
    pub agent: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
}

impl Default for Headers {
    fn default() -> Self {
        Headers {
            agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:97.0) Gecko/20100101 Firefox/97.0".to_string(),
            accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            accept_language: "ru".to_string()
        }
    }
}

/// A database of headers we can use to imitate random browsers
pub struct HeaderDB {
    fallback: Headers,
    data: Vec<Headers>,
}

impl HeaderDB {
    /// Create the header database
    pub fn new() -> HeaderDB {
        HeaderDB {
            fallback: Headers::default(),
            data: HeaderDB::default_data(),
        }
    }

    /// Setup a database of random headers
    fn default_data() -> Vec<Headers> {
        //TODO generate this
        // https://developers.whatismybrowser.com/useragents/explore/software_type_specific/web-browser/
        vec![
            Headers { agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:97.0) Gecko/20100101 Firefox/97.0".to_string(), accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string(), accept_encoding: "gzip, deflate, br".to_string(), accept_language: "ru".to_string() },
            Headers { agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".to_string(), accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string(), accept_encoding: "gzip, deflate, br".to_string(), accept_language: "ru".to_string() },
            Headers { agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36 Edg/96.0.1054.62".to_string(), accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string(), accept_encoding: "gzip, deflate, br".to_string(), accept_language: "ru".to_string() }
        ]
    }

    /// Returns a bunch of random headers for some request
    pub fn get_random_headers(&self) -> Headers {
        self.data
            .choose(&mut rand::thread_rng())
            .unwrap_or(&self.fallback)
            .clone()
    }
}
