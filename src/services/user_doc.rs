use mongodb::options::FindOptions;
use mongodb::results::UpdateResult;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    sync::Collection,
    sync::Database,
};
use serde::{Deserialize, Serialize};

use crate::routes::user_controller::{PageQuery, RegisterForm};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDocument {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub admin: bool,
    pub token: String,
}

impl UserDocument {
    pub fn hidden_props(mut self) -> UserDocument {
        self.token = "".to_string();
        self.password = "".to_string();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponse<T> {
    total: u64,
    data: Vec<T>,
}

fn get_user_collection(db: &Database) -> Collection<UserDocument> {
    db.collection::<UserDocument>("user")
}

#[allow(dead_code)]
impl UserDocument {
    pub fn add_one(db: &Database, data: RegisterForm) -> Option<InsertOneResult> {
        let collection = db.collection("user");
        let user = doc! {
            "username": data.username,
            "password": data.password,
            "email": data.email,
            "admin": false,
            "token": ""
        };
        return match collection.insert_one(user, None) {
            Ok(insert_result) => Some(insert_result),
            Err(_) => None,
        };
    }

    pub fn delete_one(db: &Database, id: ObjectId) -> Option<UserDocument> {
        let collection = get_user_collection(db);
        if let Some(user) = collection.find_one_and_delete(doc! {"_id": id}, None).ok() {
            return user;
        }
        None
    }

    /// mongodb bson action
    /// mongodb not type safe [如果插入了不同类型的数据,mongodb无法Serialize,查不到数据]
    /// @doc https://docs.mongodb.com/manual/reference/operator/update/positional/
    pub fn update_one(db: &Database, data: &UserDocument) -> Option<UpdateResult> {
        let collection = get_user_collection(db);
        return match collection.update_one(
            doc! {"_id": data._id},
            doc! {
                "$set": {
                    "username": data.username.to_string(),
                    "password": data.password.to_string(),
                    "email": data.email.to_string(),
                    "admin": data.admin,
                    "token": data.token.to_string(),
                }
            },
            None,
        ) {
            Ok(user) => Some(user),
            Err(err) => {
                eprintln!("update_one err : {:?}", err);
                None
            }
        };
    }

    pub fn find_by_id(db: &Database, id: ObjectId) -> Option<UserDocument> {
        let collection = get_user_collection(db);
        if let Some(doc) = collection.find_one(doc! {"_id": id }, None).ok() {
            if let Some(user) = doc {
                return Some(user.hidden_props());
            }
        }
        None
    }

    pub fn find_by_username(db: &Database, username: String) -> Option<UserDocument> {
        let collection = get_user_collection(db);
        if let Some(user) = collection.find_one(doc! {"username": username}, None).ok() {
            return user;
        }
        None
    }

    pub fn find_many(db: &Database, page_query: PageQuery) -> Option<PageResponse<UserDocument>> {
        let collection = get_user_collection(db);
        let mut find_option = FindOptions::default();
        find_option.sort = Some(doc! {"username": 1});
        find_option.skip = Some((page_query.page - 1) * page_query.page_size as u64);
        find_option.limit = Some(page_query.page_size as i64);
        if let Some(total) = collection.count_documents(None, None).ok() {
            if let Some(cursor) = collection.find(None, find_option).ok() {
                println!("find_many has cursor , and count_documents {:?}", total);
                let mut data = Vec::new();
                for user in cursor {
                    match user {
                        Ok(user) => {
                            data.push(user.hidden_props());
                            if data.len() >= page_query.page_size as usize {
                                break;
                            }
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
                return Some(PageResponse { total, data });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::base::AppConfig;

    use super::*;

    #[test]
    fn test_find_many() {
        if let Some(db) = AppConfig::new().get_database() {
            if let Some(res) = UserDocument::find_many(
                &db,
                PageQuery {
                    page: 1,
                    page_size: 10,
                },
            ) {
                assert!(res.total > 0);
                assert!(res.data.len() > 0);
                assert!(res.total >= res.data.len() as u64);
            }
        }
    }
}
