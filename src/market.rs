use crate::domain::*;
use crate::errors::Error;
use crate::TinkoffInvestClient;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::collections::HashMap;

#[async_trait]
pub trait Market {
    async fn stocks(&self) -> Result<Response<MarketInstrumentListPayload>, Error>;

    async fn bonds(&self) -> Result<Response<MarketInstrumentListPayload>, Error>;

    async fn etfs(&self) -> Result<Response<MarketInstrumentListPayload>, Error>;

    async fn currencies(&self) -> Result<Response<MarketInstrumentListPayload>, Error>;

    async fn order_book(&self, figi: &str, depth: i32)
        -> Result<Response<OrderBookPayload>, Error>;

    async fn candles(
        &self,
        figi: &str,
        from: &DateTime<Local>,
        to: &DateTime<Local>,
        interval: &Interval,
    ) -> Result<Response<CandlesPayload>, Error>;

    async fn search_by_figi(
        &self,
        figi: &str,
    ) -> Result<Response<SearchMarketInstrumentPayload>, Error>;

    async fn search_by_ticker(
        &self,
        ticker: &str,
    ) -> Result<Response<MarketInstrumentListPayload>, Error>;
}

#[async_trait]
impl Market for TinkoffInvestClient {
    async fn stocks(&self) -> Result<Response<MarketInstrumentListPayload>, Error> {
        let query = HashMap::new();
        self.make_get_request("/market/stocks", &query).await
    }

    async fn bonds(&self) -> Result<Response<MarketInstrumentListPayload>, Error> {
        let query = HashMap::new();
        self.make_get_request("/market/bonds", &query).await
    }

    async fn etfs(&self) -> Result<Response<MarketInstrumentListPayload>, Error> {
        let query = HashMap::new();
        self.make_get_request("/market/etfs", &query).await
    }

    async fn currencies(&self) -> Result<Response<MarketInstrumentListPayload>, Error> {
        let query = HashMap::new();
        self.make_get_request("/market/currencies", &query).await
    }

    async fn order_book(
        &self,
        figi: &str,
        depth: i32,
    ) -> Result<Response<OrderBookPayload>, Error> {
        let depth = format!("{}", depth);

        let mut query = HashMap::new();
        query.insert("figi", figi);
        query.insert("depth", &depth);

        self.make_get_request("/market/orderbook", &query).await
    }

    async fn candles(
        &self,
        figi: &str,
        from: &DateTime<Local>,
        to: &DateTime<Local>,
        interval: &Interval,
    ) -> Result<Response<CandlesPayload>, Error> {
        let mut query = HashMap::new();
        let from = from.to_rfc3339();
        let to = to.to_rfc3339();
        let interval = interval.to_string();
        query.insert("figi", figi);
        query.insert("from", &from);
        query.insert("to", &to);
        query.insert("interval", &interval);

        self.make_get_request("/market/candles", &query).await
    }

    async fn search_by_figi(
        &self,
        figi: &str,
    ) -> Result<Response<SearchMarketInstrumentPayload>, Error> {
        let mut query = HashMap::new();
        query.insert("figi", figi);

        self.make_get_request("/market/search/by-figi", &query)
            .await
    }

    async fn search_by_ticker(
        &self,
        ticker: &str,
    ) -> Result<Response<MarketInstrumentListPayload>, Error> {
        let mut query = HashMap::new();
        query.insert("ticker", ticker);

        self.make_get_request("/market/search/by-ticker", &query)
            .await
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::Interval;
    use crate::market::Market;
    use crate::TinkoffInvestClient;
    use chrono::{Local, TimeZone};
    use mockito;
    use mockito::Matcher;

    #[tokio::test]
    async fn stocks() {
        let mock = mockito::mock("GET", "/market/stocks")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::Missing)
            .with_body(
                "{
                        \"trackingId\": \"tracking_id_0\",
                        \"status\": \"Ok\",
                        \"payload\": {
                            \"total\": 1,
                            \"instruments\": [
                                {
                                    \"figi\": \"figi_0\",
                                    \"ticker\": \"ticker_0\",
                                    \"isin\": \"isin_0\",
                                    \"minPriceIncrement\": 1.23,
                                    \"lot\": 10,
                                    \"currency\": \"RUB\",
                                    \"name\": \"name_0\",
                                    \"type\": \"Stock\"
                                },
                                {
                                    \"figi\": \"figi_1\",
                                    \"ticker\": \"ticker_1\",
                                    \"lot\": 123,
                                    \"name\": \"name_0\",
                                    \"type\": \"Stock\"
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.stocks().await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn bonds() {
        let mock = mockito::mock("GET", "/market/bonds")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::Missing)
            .with_body(
                "{
                        \"trackingId\": \"tracking_id_0\",
                        \"status\": \"Ok\",
                        \"payload\": {
                            \"total\": 1,
                            \"instruments\": [
                                {
                                    \"figi\": \"figi_1\",
                                    \"ticker\": \"ticker_1\",
                                    \"lot\": 123,
                                    \"name\": \"name_0\",
                                    \"type\": \"Bond\"
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.bonds().await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn etfs() {
        let mock = mockito::mock("GET", "/market/etfs")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::Missing)
            .with_body(
                "{
                        \"trackingId\": \"tracking_id_0\",
                        \"status\": \"Ok\",
                        \"payload\": {
                            \"total\": 1,
                            \"instruments\": [
                                {
                                    \"figi\": \"figi_1\",
                                    \"ticker\": \"ticker_1\",
                                    \"lot\": 123,
                                    \"name\": \"name_0\",
                                    \"type\": \"Etf\"
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.etfs().await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn currencies() {
        let mock = mockito::mock("GET", "/market/currencies")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::Missing)
            .with_body(
                "{
                        \"trackingId\": \"tracking_id_0\",
                        \"status\": \"Ok\",
                        \"payload\": {
                            \"total\": 1,
                            \"instruments\": [
                                {
                                    \"figi\": \"figi_1\",
                                    \"ticker\": \"ticker_1\",
                                    \"lot\": 123,
                                    \"name\": \"name_0\",
                                    \"type\": \"Currency\"
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.currencies().await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn order_book_full() {
        let mock = mockito::mock("GET", "/market/orderbook")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("figi".to_string(), "figi_0".to_string()),
                Matcher::UrlEncoded("depth".to_string(), "1".to_string()),
            ]))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"figi\": \"figi_0\",
                        \"depth\": 1,
                        \"bids\": [
                            {
                                \"price\": 123.45,
                                \"quantity\": 10
                            }
                        ],
                        \"asks\": [
                            {
                                \"price\": 67.89,
                                \"quantity\": 1
                            }
                        ],
                        \"tradeStatus\": \"NormalTrading\",
                        \"minPriceIncrement\": 1.23,
                        \"faceValue\": 2.34,
                        \"lastPrice\": 3.45,
                        \"closePrice\": 4.56,
                        \"limitUp\": 5.67,
                        \"limitDown\": 6.78
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.order_book("figi_0", 1).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn order_book_reduced() {
        let mock = mockito::mock("GET", "/market/orderbook")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("figi".to_string(), "figi_0".to_string()),
                Matcher::UrlEncoded("depth".to_string(), "1".to_string()),
            ]))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"figi\": \"figi_0\",
                        \"depth\": 1,
                        \"bids\": [
                            {
                                \"price\": 123.45,
                                \"quantity\": 10
                            }
                        ],
                        \"asks\": [
                            {
                                \"price\": 67.89,
                                \"quantity\": 1
                            }
                        ],
                        \"tradeStatus\": \"NotAvailableForTrading\",
                        \"minPriceIncrement\": 1.23
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.order_book("figi_0", 1).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn candles() {
        let mock = mockito::mock("GET", "/market/candles")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("figi".to_string(), "figi_0".to_string()),
                Matcher::UrlEncoded("interval".to_string(), "1min".to_string()),
                Matcher::UrlEncoded("from".to_string(), "2020-01-01T00:00:00+03:00".to_string()),
                Matcher::UrlEncoded("to".to_string(), "2020-01-01T00:00:00+03:00".to_string()),
            ]))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"figi\": \"figi_0\",
                        \"interval\": \"1min\",
                        \"candles\": [
                            {
                                \"figi\": \"figi_0\",
                                \"interval\": \"1min\",
                                \"o\": 1.23,
                                \"c\": 2.34,
                                \"h\": 3.45,
                                \"l\": 4.56,
                                \"v\": 5,
                                \"time\": \"2020-01-01T00:00:00.131642+03:00\"
                            }
                        ]
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        let from = Local
            .datetime_from_str("2020-01-01T00:00:00+03:00", "%Y-%m-%dT%H:%M:%S%:z")
            .unwrap();
        let to = Local
            .datetime_from_str("2020-01-01T00:00:00+03:00", "%Y-%m-%dT%H:%M:%S%:z")
            .unwrap();
        tinkoff
            .candles("figi_0", &from, &to, &Interval::_1min)
            .await
            .unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn search_by_figi_full() {
        let mock = mockito::mock("GET", "/market/search/by-figi")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::UrlEncoded(
                "figi".to_string(),
                "figi_0".to_string(),
            ))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"figi\": \"figi_0\",
                        \"ticker\": \"ticker_0\",
                        \"isin\": \"isin_0\",
                        \"minPriceIncrement\": 1.23,
                        \"lot\": 10,
                        \"currency\": \"RUB\",
                        \"name\": \"name_0\",
                        \"type\": \"Stock\"
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.search_by_figi("figi_0").await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn search_by_figi_reduced() {
        let mock = mockito::mock("GET", "/market/search/by-figi")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::UrlEncoded(
                "figi".to_string(),
                "figi_0".to_string(),
            ))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"figi\": \"figi_0\",
                        \"ticker\": \"ticker_0\",
                        \"lot\": 10,
                        \"name\": \"name_0\",
                        \"type\": \"Etf\"
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.search_by_figi("figi_0").await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn search_by_ticker_full() {
        let mock = mockito::mock("GET", "/market/search/by-ticker")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::UrlEncoded(
                "ticker".to_string(),
                "ticker_0".to_string(),
            ))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"total\": 1,
                        \"instruments\": [
                            {
                                \"figi\": \"figi_0\",
                                \"ticker\": \"ticker_0\",
                                \"isin\": \"isin_0\",
                                \"minPriceIncrement\": 1.23,
                                \"lot\": 10,
                                \"currency\": \"RUB\",
                                \"name\": \"name_0\",
                                \"type\": \"Stock\"
                            }
                        ]
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.search_by_ticker("ticker_0").await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn search_by_ticker_reduced() {
        let mock = mockito::mock("GET", "/market/search/by-ticker")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::UrlEncoded(
                "ticker".to_string(),
                "ticker_0".to_string(),
            ))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"total\": 1,
                        \"instruments\": [
                            {
                                \"figi\": \"figi_0\",
                                \"ticker\": \"ticker_0\",
                                \"lot\": 10,
                                \"name\": \"name_0\",
                                \"type\": \"Etf\"
                            }
                        ]
                    }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.search_by_ticker("ticker_0").await.unwrap();

        mock.assert();
    }
}
