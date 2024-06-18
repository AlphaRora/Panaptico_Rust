use azure_storage::core::prelude::*;  
use azure_storage_blobs::prelude::*;  
use std::sync::Arc;  
use azure_core::new_http_client;  
use std::error::Error;  
  
pub struct AzureDataLakeClient {  
    client: Arc<StorageAccountClient>,  
    container_name: String,  
}  
  
impl AzureDataLakeClient {  
    pub fn new(account: &str, access_key: &str, container_name: &str) -> Self {  
        let http_client = new_http_client();  
        let client = StorageAccountClient::new_access_key(http_client, account, access_key);  
        AzureDataLakeClient {  
            client,  
            container_name: container_name.to_string(),  
        }  
    }  
  
    pub async fn upload(&self, blob_name: &str, data: &str) -> Result<(), Box<dyn Error + Send + Sync>> {  
        let container_client = self.client.as_container_client(&self.container_name);  
        let blob_client = container_client.as_blob_client(blob_name);  
  
        blob_client.put_block_blob(data.to_string()).execute().await?;  
        Ok(())  
    }  
}  
