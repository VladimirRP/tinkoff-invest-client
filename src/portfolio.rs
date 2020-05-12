use crate::domain::*;
use crate::errors::Error;
use crate::TinkoffInvestClient;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Portfolio {
    async fn portfolio(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<PortfolioPayload>, Error>;

    async fn portfolio_currencies(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<PortfolioCurrenciesPayload>, Error>;
}

#[async_trait]
impl Portfolio for TinkoffInvestClient {
    async fn portfolio(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<PortfolioPayload>, Error> {
        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        self.make_get_request("/portfolio", &query).await
    }

    async fn portfolio_currencies(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<PortfolioCurrenciesPayload>, Error> {
        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        self.make_get_request("/portfolio/currencies", &query).await
    }
}

#[cfg(test)]
mod tests {

    use crate::portfolio::Portfolio;
    use crate::TinkoffInvestClient;
    use mockito;
    use mockito::Matcher;

    #[tokio::test]
    async fn portfolio() {
        let mock = mockito::mock("GET", "/portfolio")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::UrlEncoded(
                "brokerAccountId".to_string(),
                "account_123".to_string(),
            ))
            .with_body(
                "{
                        \"trackingId\": \"tracking_id_0\",
                        \"status\": \"Ok\",
                        \"payload\": {
                            \"positions\": [
                                {
                                    \"figi\": \"figi_0\",
                                    \"ticker\": \"ticker_0\",
                                    \"isin\": \"isin_0\",
                                    \"instrumentType\": \"Stock\",
                                    \"balance\": 12.34,
                                    \"blocked\": 0.1234,
                                    \"expectedYield\": {
                                        \"currency\": \"RUB\",
                                        \"value\": 123
                                    },
                                    \"lots\": 10,
                                    \"averagePositionPrice\": {
                                        \"currency\": \"RUB\",
                                        \"value\": 1000.01
                                    },
                                    \"averagePositionPriceNoNkd\": {
                                        \"currency\": \"RUB\",
                                        \"value\": 12345
                                    },
                                    \"name\": \"name_0\"
                                },
                                {
                                    \"figi\": \"figi_1\",
                                    \"instrumentType\": \"Etf\",
                                    \"balance\": 12.34,
                                    \"lots\": 10,
                                    \"name\": \"name_1\"
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.portfolio(Some("account_123")).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn portfolio_currencies() {
        let mock = mockito::mock("GET", "/portfolio/currencies")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::UrlEncoded(
                "brokerAccountId".to_string(),
                "account_123".to_string(),
            ))
            .with_body(
                "{
                        \"trackingId\": \"tracking_id_0\",
                        \"status\": \"Ok\",
                        \"payload\": {
                            \"currencies\": [
                                {
                                    \"currency\": \"RUB\",
                                    \"balance\": 123.45,
                                    \"blocked\": 10.01
                                },
                                {
                                    \"currency\": \"USD\",
                                    \"balance\": 678.91
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff
            .portfolio_currencies(Some("account_123"))
            .await
            .unwrap();

        mock.assert();
    }
}
