use anyhow::Result;

use crate::Client;

pub struct ConnectSecret {
    pub client: Client,
}

impl ConnectSecret {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ConnectSecret { client }
    }

    /**
     * Deletes the connect HMAC Secret for specified account.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/connect/secret/{keyId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `key_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn connect_hmac_delete_secret(&self, account_id: &str, key_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/connect/secret/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&key_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
