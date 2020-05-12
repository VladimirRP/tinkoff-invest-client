use crate::domain::*;
use crate::errors::Error;
use crate::TinkoffInvestClient;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Orders {
    async fn orders(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<Vec<OrdersPayload>>, Error>;

    async fn make_limit_order(
        &self,
        figi: &str,
        broker_account_id: Option<&str>,
        operation: Operation,
        lots: i32,
        price: f64,
    ) -> Result<Response<LimitOrderPayload>, Error>;

    async fn make_market_order(
        &self,
        figi: &str,
        broker_account_id: Option<&str>,
        operation: Operation,
        lots: i32,
    ) -> Result<Response<MarketOrderPayload>, Error>;

    async fn cancel_order(
        &self,
        order_id: &str,
        broker_account_id: Option<&str>,
    ) -> Result<Response<EmptyPayload>, Error>;
}

#[async_trait]
impl Orders for TinkoffInvestClient {
    async fn orders(
        &self,
        broker_account_id: Option<&str>,
    ) -> Result<Response<Vec<OrdersPayload>>, Error> {
        let mut query = HashMap::new();

        broker_account_id.iter().for_each(|a| {
            query.insert("brokerAccountId", *a);
        });

        self.make_get_request("/orders", &query).await
    }

    async fn make_limit_order(
        &self,
        figi: &str,
        broker_account_id: Option<&str>,
        operation: Operation,
        lots: i32,
        price: f64,
    ) -> Result<Response<LimitOrderPayload>, Error> {
        let request = OrderRequest::MakeLimitOrder {
            operation,
            lots,
            price,
        };

        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        query.insert("figi", &figi);

        let body = serde_json::to_string(&request)?;

        self.make_post_request("/orders/limit-order", &query, &body)
            .await
    }

    async fn make_market_order(
        &self,
        figi: &str,
        broker_account_id: Option<&str>,
        operation: Operation,
        lots: i32,
    ) -> Result<Response<MarketOrderPayload>, Error> {
        let request = OrderRequest::MakeMarketOrder { operation, lots };

        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        query.insert("figi", &figi);

        let body = serde_json::to_string(&request)?;

        self.make_post_request("/orders/market-order", &query, &body)
            .await
    }

    async fn cancel_order(
        &self,
        order_id: &str,
        broker_account_id: Option<&str>,
    ) -> Result<Response<EmptyPayload>, Error> {
        let mut query = HashMap::new();

        if let Some(account) = broker_account_id {
            query.insert("brokerAccountId", account);
        }

        query.insert("orderId", &order_id);

        self.make_post_request("/orders/cancel", &query, "").await
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::*;
    use crate::orders::Orders;
    use crate::TinkoffInvestClient;
    use mockito;
    use mockito::Matcher;

    #[tokio::test]
    async fn test_orders() {
        let mock = mockito::mock("GET", "/orders")
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
                        \"trackingId\":\"tracking_id_0\",
                        \"status\":\"Ok\",
                        \"payload\":[
                            {
                                \"orderId\":\"order_0\",
                                \"figi\":\"figi_0\",
                                \"operation\":\"Buy\",
                                \"status\":\"New\",
                                \"requestedLots\":10,
                                \"executedLots\":2,
                                \"type\":\"Limit\",
                                \"price\":10.10
                            },
                            {
                                \"orderId\":\"order_1\",
                                \"figi\":\"figi_1\",
                                \"operation\":\"Sell\",
                                \"status\":\"Cancelled\",
                                \"requestedLots\":3,
                                \"executedLots\":0,
                                \"type\":\"Market\",
                                \"price\":0.111
                            }]
                        }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.orders(Some("account_123")).await.unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn test_make_limit_order() {
        let mock = mockito::mock("POST", "/orders/limit-order")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_header("content-length", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("brokerAccountId".to_string(), "account_123".to_string()),
                Matcher::UrlEncoded("figi".to_string(), "figi_0".to_string()),
            ]))
            .match_body(Matcher::JsonString(
                "{\"operation\":\"Buy\",\"lots\":10,\"price\":12.34}".to_string(),
            ))
            .with_body(
                "{\
                            \"trackingId\":\"tracking_id_0\",
                            \"status\":\"Ok\",
                            \"payload\":{
                                \"orderId\":\"order_0\",
                                \"operation\":\"Buy\",
                                \"status\":\"Rejected\",
                                \"rejectReason\":\"some reason\",
                                \"message\":\"some message\",
                                \"requestedLots\":10,
                                \"executedLots\":0,
                                \"commission\": {
                                    \"currency\":\"RUB\",
                                    \"value\":0\
                                }\
                            }\
                      }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff
            .make_limit_order("figi_0", Some("account_123"), Operation::Buy, 10, 12.34)
            .await
            .unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn test_make_market_order() {
        let mock = mockito::mock("POST", "/orders/market-order")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_header("content-length", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("brokerAccountId".to_string(), "account_123".to_string()),
                Matcher::UrlEncoded("figi".to_string(), "figi_0".to_string()),
            ]))
            .match_body(Matcher::JsonString(
                "{\"operation\":\"Buy\",\"lots\":10}".to_string(),
            ))
            .with_body(
                "{\
                            \"trackingId\":\"tracking_id_0\",
                            \"status\":\"Ok\",
                            \"payload\":{
                                \"orderId\":\"order_0\",
                                \"operation\":\"Buy\",
                                \"status\":\"Rejected\",
                                \"rejectReason\":\"some reason\",
                                \"message\":\"some message\",
                                \"requestedLots\":10,
                                \"executedLots\":0,
                                \"commission\": {
                                    \"currency\":\"RUB\",
                                    \"value\":0\
                                }\
                            }\
                      }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff
            .make_market_order("figi_0", Some("account_123"), Operation::Buy, 10)
            .await
            .unwrap();

        mock.assert();
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let mock = mockito::mock("POST", "/orders/cancel")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", "Bearer token123")
            .match_header("accept", Matcher::Any)
            .match_header("host", Matcher::Any)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("brokerAccountId".to_string(), "account_123".to_string()),
                Matcher::UrlEncoded("orderId".to_string(), "order_0".to_string()),
            ]))
            .match_body("")
            .with_body(
                "{
                        \"trackingId\": \"tracking_0\",
                        \"payload\": {},
                        \"status\": \"Ok\"
                       }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff
            .cancel_order("order_0", Some("account_123"))
            .await
            .unwrap();

        mock.assert();
    }
}
