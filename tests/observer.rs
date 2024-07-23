use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use spark_orm::model::observer::Observer;
use spark_orm::model::{Model, MongodbResult};
use spark_orm::Spark;
use spark_orm_derive::Model;

// implement by adding observer to Model
#[Model(coll_name = "users", observer)]
#[derive(Serialize, Deserialize, Debug, Default)]
struct User {
    name: String,
}

#[Model(coll_name = "persons", observer)]
#[derive(Serialize, Deserialize, Debug, Default)]
struct Person {
    name: String,
}

#[Model(coll_name = "jobs")]
#[derive(Serialize, Deserialize, Debug, Default)]
struct Jobs {
    person: Person,
}

#[allow(clippy::assigning_clones)]
impl Observer<Person> for Person {
    async fn created(model: &mut Model<'_, Person>) -> MongodbResult<()> {
        let mut jobs = Jobs::new_model(None);
        jobs.person.name = model.name.clone();
        jobs.save(None).await?;
        Ok(())
    }

    async fn updated(model: &mut Model<'_, Person>) -> MongodbResult<()> {
        Ok(())
    }

    async fn deleted(model: &mut Model<'_, Person>) -> MongodbResult<()> {
        Ok(())
    }
}

impl Observer<User> for User {
    async fn created(model: &mut Model<'_, User>) -> MongodbResult<()> {
        let mut person_model = Person::new_model(None);
        if model.name == "Hello".to_string() {
            model.name = "Something".to_string();
            model.save(None).await?;
        }
        person_model.name = "Naruto".to_string();
        person_model.save(None).await?;
        Ok(())
    }
}

#[tokio::test]
async fn save() {
    connect_db().await;
    let mut user_model = User::new_model(None);
    user_model.name = "THE NEW GENERATION".to_string();
    user_model.save(None).await.unwrap();
}

async fn connect_db() {
    Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await;
}
