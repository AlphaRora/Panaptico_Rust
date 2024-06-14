// azure_storage_client.rs
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use std::sync::Arc;

pub struct AzureDataLakeClient {
    account: String,
    container: String,
    client: Arc<StorageClient>,
}

impl AzureDataLakeClient {
    pub fn new(account: &str, container: &str, access_key: &str) -> Self {
        let client = StorageClient::new_access_key(account, access_key);
        Self {
            account: account.to_string(),
            container: container.to_string(),
            client: Arc::new(client),
        }
    }

    pub async fn upload(&self, file_name: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        let container_client = self.client.as_container_client(&self.container);
        let blob_client = container_client.as_blob_client(file_name);

        let response = blob_client
            .put_block_blob(data.as_bytes().to_vec())
            .into_future()
            .await?;

        println!("Uploaded blob: {:#?}", response);
        Ok(())
    }
}
