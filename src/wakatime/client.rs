use reqwest::Client;

pub struct WakaTimeClient {
    pub(crate) client: Client,
    pub(crate) api_key: String,
}

impl WakaTimeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}
