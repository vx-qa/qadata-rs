use std::path::Prefix::Verbatim;

use mongodb::{Client, options::ClientOptions};
use mongodb::Database;
use mongodb::options::FindOptions;

use bson::{Bson, oid};
use bson::{bson, doc};
use bson::Document;

use crate::qautil::{future_day, future_min, stock_day, stock_min};

pub struct QAMongoClient {
    uri: String,
    database: Database,
}

impl QAMongoClient {
    pub fn new(uri: &str, database: &str) -> Self {
        let mut client_options = ClientOptions::parse(uri).unwrap();
        client_options.app_name = Some("QUANTAXIS".to_string());
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(database);
        Self {
            uri: uri.to_string(),
            database: db,
        }
    }

    pub fn get_stock_day(&mut self, code: &str, start: &str, end: &str) -> Vec<stock_day> {
        let collection = self.database.collection("stock_day");

        let filter = doc! {"code": code,
                                            "date": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().build();
        let cursor = collection.find(filter, find_options).unwrap();
        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: stock_day = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }

    pub fn get_stock_min(&mut self, code: &str, start: &str, end: &str, frequence: &str) -> Vec<stock_min> {
        let collection = self.database.collection("stock_min");

        let filter = doc! {"code":code, "type": frequence, "datetime": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().build();
        let cursor = collection.find(filter, find_options).unwrap();

        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: stock_min = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }
    pub fn get_future_day(&mut self, code: &str, start: &str, end: &str) -> Vec<future_day> {
        let collection = self.database.collection("future_day");

        let filter = doc! {"code": code,
                                            "date": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().build();
        let cursor = collection.find(filter, find_options).unwrap();
        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: future_day = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }

    pub fn get_future_min(&mut self, code: &str, start: &str, end: &str, frequence: &str) -> Vec<future_min> {
        let collection = self.database.collection("future_min");

        let filter = doc! {"code":code, "type": frequence, "datetime": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().build();
        let cursor = collection.find(filter, find_options).unwrap();

        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: future_min = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }
}

