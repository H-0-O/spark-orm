# Spark-ORM: MongoDB ORM for Rust

Spark-ORM is a high-performance, open-source Object-Relational Mapping (ORM) library designed specifically for MongoDB in Rust. It seamlessly bridges Rust structs with MongoDB collections, effortlessly converting structs into models.

## Features

- **Derive Models**: Effortlessly convert Rust structs into MongoDB models using the `Model` trait derivation.

- **Custom Collection Names**: Tailor collection names for your models with the `#[coll_name]` attribute.

- **Memory Efficiency**: Built for speed and memory efficiency, Spark-ORM offers a non-heap copy ORM solution for MongoDB.

## Getting Started

1. Define your model by simply applying the `Model` attribute and setting the collection name with `coll_name`:

    ```rust
   #[Model(coll_name = "users")]
   #[derive(Serialize, Deserialize, Default, Debug)]
   struct User {
    age: u32,
    name: String,
    email: String,
   }
    ```

2. Connect to the database in one of two ways:

   a. Establish a global connection:
      ```rust
      Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await;
      ```

   b. Or connect locally:
      ```rust
      Spark::connect("root", "123", "localhost", "6789", "rm_orm_db").await;
      ```

   For the global connection, Spark retains it throughout the program, accessible via: ``Spark::get_db();``

## Usage 

Instantiate the model:

```rust
    let mut user = User::new_model(&db);
```
Update attributes:

```rust
   user.name = "Hossein".to_string();
   user.age = 22;
```

Save to the database:


```rust
   user.save().await.unwrap();
```

---

Find a model:


```rust
       let mut user = User::new_model(&db);
       let sample = doc! {
            "name" : "Hossein",
            "email" : "spark_orm_test"
       };
       user.find_one(Prototype::Doc(sample)).await.unwrap().unwrap();
       println!("{}" , user.name);
```

---

Update and save:


```rust
    let mut user = User::new_model(&db);
    user.name = "Hossein".to_string();
    user.email = "spark_orm_test".to_string();

    user.save().await;

    user.name = "Nothing".to_string();

    user.update().await;
```

--- 
Delete a record:


```rust
        let mut user = User::new_model(&db);
        user.delete().await;
```

## Attributes

Define index or unique attributes for struct fields:


```rust
   #[Model(coll_name = "products")]
   #[derive(Serialize, Deserialize, Default, Debug)]
   struct Product {
       #[index]
       age: u32,
       #[index]
       name: String,
       #[unique]
       email: String,
   }

```

These indexes are registered during the first initiation of Product.

