use crate::account::*;
use crate::market::*;
use crate::general::*;
use crate::futures::general::*;
use crate::futures::market::*;
use crate::userstream::*;
use crate::client::*;

static API_HOST: &str = "https://api.binance.us";
static FAPI_HOST: &str = "https://fapi.binance.us";

//#[derive(Clone)]
pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        General {
            client: Client::new(api_key, secret_key, API_HOST.to_string()),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Account {
            client: Client::new(api_key, secret_key, API_HOST.to_string()),
            recv_window: 5000,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Market {
            client: Client::new(api_key, secret_key, API_HOST.to_string()),
            recv_window: 5000,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, API_HOST.to_string()),
            recv_window: 5000,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(api_key, secret_key, FAPI_HOST.to_string()),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(api_key, secret_key, FAPI_HOST.to_string()),
            recv_window: 5000,
        }
    }
}
