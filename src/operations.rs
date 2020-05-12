use crate::domain::*;
use crate::errors::Error;
use crate::TinkoffInvestClient;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::collections::HashMap;

#[async_trait]
pub trait Operations {
    async fn operations(
        &self,
        from: &DateTime<Local>,
        to: &DateTime<Local>,
        figi: Option<&str>,
        broker_account_id: Option<&str>,
    ) -> Result<Response<OperationsPayload>, Error>;
}

#[async_trait]
impl Operations for TinkoffInvestClient {
    async fn operations(
        &self,
        from: &DateTime<Local>,
        to: &DateTime<Local>,
        figi: Option<&str>,
        broker_account_id: Option<&str>,
    ) -> Result<Response<OperationsPayload>, Error> {
        let mut query = HashMap::new();

        let from = from.to_rfc3339();
        let to = to.to_rfc3339();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        if let Some(f) = figi {
            query.insert("figi", f);
        }

        query.insert("from", &from);
        query.insert("to", &to);

        self.make_get_request("/operations", &query).await
    }
}

#[cfg(test)]
mod tests {

    use crate::operations::Operations;
    use crate::TinkoffInvestClient;
    use chrono::{Local, TimeZone};
    use mockito;
    use mockito::Matcher;

    #[tokio::test]
    async fn operations() {
        let mock = mockito::mock("GET", "/operations")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("brokerAccountId".to_string(), "account_123".to_string()),
                Matcher::UrlEncoded("figi".to_string(), "figi_0".to_string()),
                Matcher::UrlEncoded("from".to_string(), "2020-01-01T00:00:00+03:00".to_string()),
                Matcher::UrlEncoded("to".to_string(), "2020-01-01T00:00:00+03:00".to_string()),
            ]))
            .with_body(
                "{
                    \"trackingId\": \"tracking_id_0\",
                    \"status\": \"Ok\",
                    \"payload\": {
                        \"operations\": [
                            {
                                \"id\": \"operation_0\",
                                \"status\": \"Done\",
                                \"trades\": [
                                    {
                                        \"tradeId\": \"trade_0\",
                                        \"date\": \"2020-02-01T18:38:33.131642+03:00\",
                                        \"price\": 123.45,
                                        \"quantity\": 123
                                    }
                                ],
                                \"commission\": {
                                \"currency\": \"RUB\",
                                \"value\": 0.123
                            },
                            \"currency\": \"RUB\",
                            \"payment\": 123.45,
                            \"price\": 123.45,
                            \"quantity\": 10,
                            \"figi\": \"figi_0\",
                            \"instrumentType\": \"Stock\",
                            \"isMarginCall\": false,
                            \"date\": \"2020-02-01T18:38:33.131642+03:00\",
                            \"operationType\": \"Buy\"
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
            .operations(&from, &to, Some("figi_0"), Some("account_123"))
            .await
            .unwrap();

        mock.assert();
    }
}
