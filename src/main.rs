// use mongodb::bson::doc; // this is a macro to create a Document from a json
use mongodb::bson::bson; // bson macro can create bson elements from primitives

use mongodb::bson::Bson;
use mongodb::bson::Document;
// use mongodb::options::FindOptions;
use mongodb::sync::Client;
use mongodb::sync::Collection;
use mongodb::sync::Database;
mod book;

use std::result::Result;
#[allow(warnings)]
#[allow(unsued_imports)]
pub fn main() -> Result<(), mongodb::error::Error> {
    println!("Hello, rust mongo!");

    let db = get_database().unwrap();
    let collection = db.collection("books");

    // create a book dto
    let book_1_struct = book::Book::new("1984", "George Orwell");
    println!("struct book_1 = {:?}", book_1_struct);

    insert(collection, book_1_struct);


    // // from dto to json
    // let book_1_json = serde_json::to_string(&book_1_struct).unwrap();
    // println!("book_1_json = {:?}", book_1_json);

    // // from json to bson
    // let bson_from_json = bson!(&book_1_json);
    // println!("bson_from_json = {:?}", bson_from_json);

    // let bson_from_book_1_struct: Bson = bson::to_bson(&book_1_struct).unwrap();

    // println!(
    //     "bson_from_book_1_struct = {:?}",
    //     bson_from_book_1_struct.as_document().unwrap()
    // );

    // let book_1_1_struct: book::Book = serde_json::from_str(book_1_json.as_str()).unwrap();
    // println!("struct book_1_1_struct = {:?}", book_1_1_struct);
    // // verify book 1 is the same like book 1_1
    // assert_eq!(book_1_struct, book_1_1_struct);
    // println!(
    //     "struct book_1_struct eqals to book_1_1_struct = {:?}",
    //     book_1_struct == book_1_1_struct
    // );

    // let docs = vec![
    //     doc! { "title": "1984", "author": "George Orwell" },
    //     doc! { "title": "Animal Farm", "author": "George Orwell" },
    //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    // ];
    //
    // collection.insert_many(docs, None)?;

    // let find_options = FindOptions::builder().sort(doc! { "title": -1 }).build();

    // let cursor = collection.find(doc! {}, find_options)?;
    // for result in cursor {
    //     match result {
    //         Ok(document) => {
    //             println!("{:?}", document);
    //             println!("title {:?}", document.get("title"));
    //             println!("title {:?}", document.get("title").and_then(Bson::as_str));
    //         }
    //         Err(e) => return Err(e.into()),
    //     }
    // }

    return Ok(());
}

fn get_database() -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let database = client.database("rustdb");
    return Ok(database);
}

fn insert(collection: Collection, data: book::Book) -> std::result::Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    let insert_data: Bson = bson::to_bson(&data).unwrap();
    let document = insert_data.as_document().unwrap();
    println!("insert = {:?}", document);
    return collection.insert_one(document.to_owned(), None);
}
    