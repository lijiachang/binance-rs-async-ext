use crate::account::*;
use crate::client::*;
use crate::config::Config;
use crate::general::*;
use crate::market::*;
use crate::userstream::*;

pub trait Binance: Sized {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    /// Create a binance API using environment variables for credentials
    /// BINANCE_API_KEY=$YOUR_API_KEY
    /// BINANCE_API_SECRET_KEY=$YOUR_SECRET_KEY
    fn new_with_env(config: &Config) -> Self {
        let api_key = std::env::var("BINANCE_API_KEY").ok();
        let secret = std::env::var("BINANCE_API_SECRET_KEY").ok();
        Self::new_with_config(api_key, secret, config)
    }

    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}

impl Binance for General {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
        }
    }
}

impl Binance for Account {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "savings_api")]
impl Binance for crate::savings::Savings {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Market {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for UserStream {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::general::FuturesGeneral {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
                config.timeout,
            ),
        }
    }
}

#[cfg(feature = "coin_futures_api")]
impl Binance for crate::coin_futures::general::FuturesGeneral {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.coin_futures_rest_api_endpoint.clone(),
                config.timeout,
            ),
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::market::FuturesMarket {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
                config.timeout,
            ),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::account::FuturesAccount {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
                config.timeout,
            ),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::userstream::UserStream {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
                config.timeout,
            ),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "margin_api")]
impl Binance for crate::margin::Margin {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "wallet_api")]
impl Binance for crate::wallet::Wallet {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
            binance_us_api: config.binance_us_api,
        }
    }
}
