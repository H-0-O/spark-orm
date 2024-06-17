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
    let mut user = User::new_model(Some(&db));
```
OR  you can use the global connection :

```rust
 let mut user = User::new_model(None);
```
if you didn't set global connection , the new_model function will panic 

----------------------

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
        let mut user_model = User::new_model(Some(&db)); 
        let mut sample = User::default ();
        sample.name = "Hossein".to_string();
        let founded = user_model.find_one(
            sample,
            None,
            ).await.unwrap();
        println!("The founded object {:?} ", founded);
```

---

Update and save:


```rust
    let mut user = User::new_model(Some(&db));
    user.name = "Hossein".to_string();
    user.email = "spark_orm_test".to_string();

    user.save().await;

    user.name = "Nothing".to_string();

    user.save().await;
```

### or

```rust
      let db = get_db().await;
      let user_model = User::new_model(Some(&db));
      let updated = user_model.update(
      doc! {
                  "name": "Hossein",
              },
      doc! {
                  "$set": {
                      "name": "Hossein 33"
                  }
              },
      None,
      ).await.unwrap();
       println!("The Updated info {:?}", updated);
```
--- 
Delete a record:


```rust
        let mut user = User::new_model(Some(&db));
        user.delete().await;
```

Note: you can use the ``?`` instead of unwrap 

## Model

 The model trait adds _id , timestamps (created_at , updated_at , deleted_at) to your struct and fill automatically


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

### I would greatly appreciate your support on GitHub. Please consider giving me a [star](https://github.com/H-0-O/spark-orm.git) to show your support. Thank you! 
#  Note the library is under development and may have lots of changes in the future, even in its basics