use crate::client::*;
use crate::errors::*;
use crate::futures::rest_model::*;
use crate::rest_model::ServerTime;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    /// Test connectivity
    pub async fn ping(&self) -> Result<String> {
        let resp: serde_json::Value = self.client.get("/dapi/v1/ping", None).await?;
        //正常的响应应该是{}
        return if resp.is_object() && resp.as_object().unwrap().is_empty() {
            Ok("pong".into())
        } else {
            Err(Error::Msg(resp.to_string()))
        }
    }

    /// Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> { self.client.get_p("/dapi/v1/time", None).await }

    /// Obtain exchange information
    /// - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<CoinFutureExchangeInformation> {
        self.client.get_p("/dapi/v1/exchangeInfo", None).await
    }

    /// Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<CoinFutureSymbol>
        where
            S: Into<String>,
    {
        let symbol_string = symbol.into();
        let upper_symbol = symbol_string.to_uppercase();
        match self.exchange_info().await {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                Err(Error::UnknownSymbol(symbol_string.clone()))
            }
            Err(e) => Err(e),
        }
    }
}
