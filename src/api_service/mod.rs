//importaciones externas

use bson::{doc, Document};
use mongodb::results::{DeleteResult, UpdateResult, InsertOneResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};

//constructores externos

extern crate serde;
extern crate serde_json;

//estructura de datos para mongodb

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub author: String,
}

//referencia a la coleccion clonada

#[derive(Clone)]
pub struct ApiService{
    collection: Collection,
}

//transformar los datos a documento de mongo db

fn data_to_document(data: &Data) -> Document {
    let Data {
        title,
        author,
    } = data;
    doc! {
        "title": title,
        "author": author,
    }
}

//funciones con quieries a mongo

impl ApiService {
    pub fn new(collection: Collection) -> ApiService {
        ApiService { collection }
    }

    //inserta datos en mongodb
    pub fn create(&self, _data:&Data) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(data_to_document(_data), None)
    }

    //actualiza documento existente
    pub fn update(&self, _data:&Data, _param: &String) -> Result<UpdateResult, Error> {
        let object_param = bson::oid::ObjectId::with_string(_param).unwrap();
        self.collection.update_one(doc! { "_id": object_param }, data_to_document(_data), None)
    }

    //borra documento
    pub fn delete(&self, _title: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "title": _title }, None)
    }

    //ver todos los documentos
    pub fn get_json(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(None, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    //obtienes los documentos con quiery
    pub fn get_by(&self, param: &String) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(doc! { "author": { "$regex": param } }, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }
}