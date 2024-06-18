use azure_storage::core::prelude::*;  
use azure_storage_blobs::prelude::*;  
use std::error::Error;  
  
pub struct AzureDataLakeClient {  
    account: String,  
    access_key: String,  
    container_name: String,  
}  
  
impl AzureDataLakeClient {  
    pub fn new(account: &str, access_key: &str, container_name: &str) -> Self {  
        AzureDataLakeClient {  
            account: account.to_string(),  
            access_key: access_key.to_string(),  
            container_name: container_name.to_string(),  
        }  
    }  
  
    pub async fn upload(&self, blob_name: &str, data: &str) -> Result<(), Box<dyn Error>> {  
        let storage_account_client = StorageAccountClient::new_access_key(&self.account, &self.access_key);  
        let blob_service_client = storage_account_client.as_storage_client().as_blob_service_client();  
        let container_client = blob_service_client.as_container_client(&self.container_name);  
        let blob_client = container_client.as_blob_client(blob_name);  
  
        blob_client.put_block_blob(data.to_string()).await?;  
        Ok(())  
    }  
}  
