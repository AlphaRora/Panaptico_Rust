// azure_storage_client.rs
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use azure_identity::token_credentials::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

pub struct AzureDataLakeClient {
    container_client: Arc<ContainerClient>,
}

impl AzureDataLakeClient {
    pub fn new(account: &str, container: &str) -> Self {
        let storage_account_client = StorageAccountClient::new_account_sas_credentials(
            account,
            &AzureCliCredential::new().unwrap(),
        );
        let container_client = storage_account_client.as_container_client(container);

        Self {
            container_client: Arc::new(container_client),
        }
    }

    pub async fn upload(&self, file_name: &str, content: &str) -> azure_core::Result<()> {
        let blob_client = self.container_client.as_blob_client(file_name);
        let data = content.as_bytes();
        blob_client.put_block_blob(data).await?;
        Ok(())
    }
}
