# RSpark MongoDB ORM for Rust

RSpark is an open-source, fast, and memory-efficient Object-Relational Mapping (ORM) library for MongoDB written in Rust. It allows seamless integration between Rust structs and MongoDB collections by automatically converting structs to models.

## Features

- **Derive Model**: Easily convert your Rust structs to MongoDB models by deriving the `Model` trait.

- **Collection Name Customization**: Set the collection name for your models using the `#[coll_name]` attribute.

- **Non-Heap Copy**: RSpark is designed for efficiency, providing a non-heap copy ORM solution for MongoDB.

## Getting Started

1. Add RSpark as a dependency in your `Cargo.toml` file:

    ```toml
    [dependencies]
    rspark = "0.1.0"
    ```

2. Derive the `Model` trait for your structs and customize collection names using `#[coll_name]`:

    ```rust
    #[derive(Model)]
    #[coll_name = "Books"]
    pub struct Book {
        // ... fields here
    }
    ```

3. Connect to MongoDB and start using the ORM:

    ```rust
    use rspark::{RSpark, Model};

    #[tokio::main]
    async fn main() {
        // Connect to MongoDB
        let db = RSpark::connect("admin", "123", "localhost", "27019", "main_db").await;

        // Create a new Book instance
        let mut the_book = Book::new(&db).await;
        the_book.fill(/* fill with your data */);

        // Save the book
        the_book.save().await.unwrap();
    }
    ```

