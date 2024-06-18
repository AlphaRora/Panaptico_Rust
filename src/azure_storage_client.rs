use azure_storage::clients::StorageClient;
use azure_storage_blobs::prelude::*;
use std::error::Error;

pub struct AzureDataLakeClient {
    client: StorageClient,
    container_name: String,
}

impl AzureDataLakeClient {
    pub fn new(account: &str, container_name: &str) -> Self {
        let client = StorageClient::new_access_key(account, "ACCESS_KEY");
        AzureDataLakeClient {
            client,
            container_name: container_name.to_string(),
        }
    }

    pub async fn upload(&self, blob_name: &str, data: &str) -> Result<(), Box<dyn Error>> {
        let blob_client = self.client.container_client(&self.container_name).blob_client(blob_name);
        blob_client.put_block_blob(data).await?;
        Ok(())
    }
}
