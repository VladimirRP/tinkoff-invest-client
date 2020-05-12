use crate::domain::*;
use crate::errors::Error;
use crate::TinkoffInvestClient;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait User {
    async fn accounts(&self) -> Result<Response<AccountsPayload>, Error>;
}

#[async_trait]
impl User for TinkoffInvestClient {
    async fn accounts(&self) -> Result<Response<AccountsPayload>, Error> {
        let query = HashMap::new();

        self.make_get_request("/user/accounts", &query).await
    }
}

#[cfg(test)]
mod tests {

    use crate::user::User;
    use crate::TinkoffInvestClient;
    use mockito;
    use mockito::Matcher;

    #[tokio::test]
    async fn accounts() {
        let mock = mockito::mock("GET", "/user/accounts")
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
                            \"accounts\": [
                                {
                                    \"brokerAccountType\": \"Tinkoff\",
                                    \"brokerAccountId\": \"account_0\"
                                }
                            ]
                        }
                }",
            )
            .create();

        let endpoint = &mockito::server_url();
        let token = "token123";
        let tinkoff = TinkoffInvestClient::new(reqwest::Client::new(), &endpoint, &token);

        tinkoff.accounts().await.unwrap();

        mock.assert();
    }
}
