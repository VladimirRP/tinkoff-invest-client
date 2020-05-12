use crate::domain::*;
use crate::errors::Error;
use crate::TinkoffInvestClient;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Sandbox {
    async fn register(
        &self,
        broker_account_type: BrokerAccountType,
    ) -> Result<Response<SandboxAccount>, Error>;

    async fn set_currencies_balance(
        &self,
        broker_account_id: Option<&str>,
        currency: Currency,
        balance: f64,
    ) -> Result<Response<EmptyPayload>, Error>;

    async fn set_positions_balance(
        &self,
        broker_account_id: Option<&str>,
        figi: &str,
        balance: f64,
    ) -> Result<Response<EmptyPayload>, Error>;

    async fn remove(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<EmptyPayload>, Error>;

    async fn clear(&self, broker_account_id: Option<&str>)
        -> Result<Response<EmptyPayload>, Error>;
}

#[async_trait]
impl Sandbox for TinkoffInvestClient {
    async fn register(
        &self,
        broker_account_type: BrokerAccountType,
    ) -> Result<Response<SandboxAccount>, Error> {
        let request = SandboxRequest::Register {
            broker_account_type,
        };

        let body = serde_json::to_string(&request)?;

        let query = HashMap::new();

        self.make_post_request("/sandbox/register", &query, &body)
            .await
    }

    async fn set_currencies_balance(
        &self,
        broker_account_id: Option<&str>,
        currency: Currency,
        balance: f64,
    ) -> Result<Response<EmptyPayload>, Error> {
        let request = SandboxRequest::SetCurrenciesBalance { currency, balance };

        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }
        let body = serde_json::to_string(&request)?;

        self.make_post_request("/sandbox/currencies/balance", &query, &body)
            .await
    }

    async fn set_positions_balance(
        &self,
        broker_account_id: Option<&str>,
        figi: &str,
        balance: f64,
    ) -> Result<Response<EmptyPayload>, Error> {
        let figi = figi.to_string();

        let request = SandboxRequest::SetPositionBalance { figi, balance };

        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        let body = serde_json::to_string(&request)?;

        self.make_post_request("/sandbox/positions/balance", &query, &body)
            .await
    }

    async fn remove(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<EmptyPayload>, Error> {
        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        self.make_post_request("/sandbox/remove", &query, "").await
    }

    async fn clear(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<EmptyPayload>, Error> {
        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        self.make_post_request("/sandbox/clear", &query, "").await
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::*;
    use crate::sandbox::Sandbox;
    use crate::TinkoffInvestClient;
    use mockito;
    use mockito::Matcher;

    #[tokio::test]
    async fn register() {
        let mock = mockito::mock("POST", "/sandbox/register")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("content-length", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_body(Matcher::JsonString(
                "{\"brokerAccountType\":\"Tinkoff\"}".to_string(),
            ))
            .with_body(
                "{
                        \"trackingId\": \"some_tracking_id\",
                        \"status\": \"Ok\",
                        \"payload\":
                            {
                                \"brokerAccountType\": \"Tinkoff\",
                                \"brokerAccountId\": \"account_123\"
                            }
                        }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.register(BrokerAccountType::Tinkoff).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn set_currencies_balance() {
        let mock = mockito::mock("POST", "/sandbox/currencies/balance")
            .match_query(Matcher::UrlEncoded(
                "brokerAccountId".to_string(),
                "account_123".to_string(),
            ))
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("content-length", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_body(Matcher::JsonString(
                "{\"currency\":\"RUB\",\"balance\":1000.0}".to_string(),
            ))
            .with_body("{\"trackingId\": \"some_tracking_id\",\"payload\": {},\"status\": \"Ok\"}")
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff
            .set_currencies_balance(Some("account_123"), Currency::RUB, 1000.0)
            .await
            .unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn set_positions_balance() {
        let mock = mockito::mock("POST", "/sandbox/positions/balance")
            .match_query(Matcher::UrlEncoded(
                "brokerAccountId".to_string(),
                "account_123".to_string(),
            ))
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("content-length", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_body(Matcher::JsonString(
                "{\"figi\":\"some_figi\",\"balance\":1000.0}".to_string(),
            ))
            .with_body("{\"trackingId\": \"some_tracking_id\",\"payload\": {},\"status\": \"Ok\"}")
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff
            .set_positions_balance(Some("account_123"), "some_figi", 1000.0)
            .await
            .unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn remove() {
        let mock = mockito::mock("POST", "/sandbox/remove")
            .match_query(Matcher::UrlEncoded(
                "brokerAccountId".to_string(),
                "account_123".to_string(),
            ))
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_body("")
            .with_body("{\"trackingId\": \"some_tracking_id\",\"payload\": {},\"status\": \"Ok\"}")
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.remove(Some("account_123")).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn clear() {
        let mock = mockito::mock("POST", "/sandbox/clear")
            .match_query(Matcher::UrlEncoded(
                "brokerAccountId".to_string(),
                "account_123".to_string(),
            ))
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_body("")
            .with_body("{\"trackingId\": \"some_tracking_id\",\"payload\": {},\"status\": \"Ok\"}")
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.clear(Some("account_123")).await.unwrap();

        mock.assert();
    }
}
