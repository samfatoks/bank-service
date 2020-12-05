use qldb::QLDBClient;
use std::collections::HashMap;
use ion_binary_rs::IonValue;
use crate::Config;
use crate::domain::QldbInsertable;
use std::convert::TryInto;
use crate::error::Error;

pub struct QldbProcessor {
    client: QLDBClient,
}

impl QldbProcessor {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let client = QLDBClient::default(&config.ledger_name).await?;
        Ok(QldbProcessor {
            client
        })
    }

    // pub async fn insert<I: QldbInsertable>(&self, model: &I) -> Result<String, Error> {
    //     let transaction = self.client.transaction().await?;
    //     let results = transaction
    //         .query(format!("INSERT INTO {} VALUE ?", model.table_name()).as_ref())
    //         .param(model.to_params())
    //         .execute()
    //         .await?;
    //     transaction.commit().await?;
    //     let result = &results[0];
    //     let map: HashMap<String, IonValue> = result.try_into().unwrap();
    //     let document_id: String = map.get("documentId").unwrap().try_into()?;
    //     Ok(document_id)
    // }
    pub async fn insert<I: QldbInsertable>(&self, model: &I) -> Result<String, Error> {
        let results = self.client
        .transaction_within(|client| async move {   
            let results = client
                .query(format!("INSERT INTO {} VALUE ?", model.table_name()).as_ref())
                .param(model.to_params())
                .execute()
                .await?;
            Ok(results)
        }).await?;

        let result = &results[0];
        let map: HashMap<String, IonValue> = result.try_into().unwrap();
        let document_id: String = map.get("documentId").unwrap().try_into()?;
        Ok(document_id)
    }

    pub async fn transfer<I: QldbInsertable>(&self, model: &I) -> Result<(), Error> {
        self.client
        .transaction_within(|client| async move {   
            let result = client
                .query(format!("INSERT INTO {} VALUE ?", model.table_name()).as_ref())
                .param(model.to_params())
                .execute()
                .await?;
            info!("{:?}", result);
            Ok(())
        }).await?;
        Ok(())
    }

    // pub async fn query(&self, query: &str) -> Result<Vec<Account>, Box<dyn std::error::Error>> {
    //     let mut builder = self.client.read_query(query).await?;
    //     let accounts: Vec<Account> = builder.execute().await?.iter().map(|i| i.try_into()).filter_map(Result::ok).collect();
    //     Ok(accounts)
    // }
    pub async fn query(&self, query_str: &str) -> Result<Vec<IonValue>, Error> {
        let mut builder = self.client.read_query(query_str).await?;
        let results = builder.execute().await?;
        Ok(results)
    }

    pub async fn delete(&self, query_str: &str) -> Result<Vec<String>, Error> {
        let results = self.client
        .transaction_within(|client| async move {   
            let results = client
                .query(query_str)
                .execute()
                .await?;
            Ok(results)
        }).await?;

        if results.len() == 0 {
            Err(Error::NoRowsAffected)
        } else {
            let mut doc_ids = Vec::new();
            for result in &results {
                let map: HashMap<String, IonValue> = result.try_into().unwrap();
                let document_id: String = map.get("documentId").unwrap().try_into()?;
                doc_ids.push(document_id);
            }

            Ok(doc_ids)
        }
    }

}