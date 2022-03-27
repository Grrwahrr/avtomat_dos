use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Headers {
    pub agent: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
}

impl Default for Headers {
    fn default() -> Self {
        Self {
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
    pub fn new() -> Self {
        Self {
            fallback: Headers::default(),
            data: HeaderDB::default_data(),
        }
    }

    /// Setup a database of random headers
    /// https://developers.whatismybrowser.com/useragents/explore/software_type_specific/web-browser/
    /// https://user-agents.net/browsers
    fn default_data() -> Vec<Headers> {
        let mut data = vec![];

        for agent in HeaderDB::default_agents() {
            for accept in HeaderDB::default_accept() {
                for accept_encoding in HeaderDB::default_accept_encoding() {
                    for accept_language in HeaderDB::default_accept_language() {
                        data.push(Headers {
                            agent: agent.clone(),
                            accept: accept.clone(),
                            accept_encoding: accept_encoding.clone(),
                            accept_language,
                        });
                    }
                }
            }
        }

        data
    }

    fn default_agents() -> Vec<String> {
        vec![
            "Mozilla/5.0 (X11; Linux x86_64; en-GB; rv:104.0esr) Gecko/20170614 Firefox/104.0esr".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12; rv:104.0esr) Gecko/20010101 Firefox/104.0esr".to_string(),
            "Mozilla/5.0 (Windows NT 6.2; Win64; x64; rv:104.0) Gecko/20100101 Goanna/4.6 Firefox/104.0 Mypal/28.11.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_7; rv:104.0) Gecko/20010101 Firefox/104.0".to_string(),
            "Mozilla/5.0 (X11; U; Linux x86_64; rv:104.0) Gecko/20000409 Firefox/104.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_0; rv:104.0) Gecko/20110101 Firefox/104.0".to_string(),

            "Mozilla/5.0 (Linux; Android 8.1.0; SM-A260F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.30 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.46 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.46 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.42 Whale/3.14.133.9 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_11) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4863.128 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; CrOS x86_64 14526.11.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.20 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4847.170 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; CrOS x86_64 14526.28.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.44 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4889.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; Android 11; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.46 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.45 Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; Android 11; LM-G820N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.30 Mobile Safari/537.36".to_string(),

            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4684.2 YaBrowser/21.9.2.169 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4678.0 YaBrowser/21.9.1.686 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4663.176 YaBrowser/21.9.2.169 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; arm; Android 6.0; LG-H422) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4648.146 YaBrowser/21.9.0.359.00 SA/3 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; arm_64; Android 11; M2102J20SG) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 YaBrowser/21.9.0.359.00 SA/3 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.42 YaBrowser/21.9.2.169 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4687.83 YaBrowser/21.9.2.172 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; arm_64; Android 11; SM-A217F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 YaBrowser/21.9.0.359.00 SA/3 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; arm_64; Android 11; M2101K9AG) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4656.1 YaBrowser/21.9.0.359.00 SA/3 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4651.128 YaBrowser/21.9.2.169 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4652.105 YaBrowser/21.9.2.169 Yowser/2.5 Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; arm_64; Android 8.1.0; SM-T835) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 YaBrowser/21.9.0.359.01 Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; arm_64; Android 12; Pixel 4 XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4688.3 YaBrowser/21.9.0.359.00 SA/3 Mobile Safari/537.36".to_string(),

            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36 OPR/83.0.4254.27".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36 OPR/83.0.4254.70/mkN49kSV-12".to_string(),
            "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36 OPR/83.0.4254.27 (Edition Campaign 42)".to_string(),
        ]
    }

    fn default_accept() -> Vec<String> {
        vec![
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
                .to_string(),
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
        ]
    }

    fn default_accept_encoding() -> Vec<String> {
        vec![
            "gzip, deflate, br".to_string(),
            // "gzip,deflate".to_string()
        ]
    }

    fn default_accept_language() -> Vec<String> {
        vec!["ru-RU".to_string(), "ru,en-us;q=0.7,en;q=0.3".to_string()]
    }

    /// Returns a bunch of random headers for some request
    pub fn get_random_headers(&self) -> Headers {
        self.data
            .choose(&mut rand::thread_rng())
            .unwrap_or(&self.fallback)
            .clone()
    }
}
