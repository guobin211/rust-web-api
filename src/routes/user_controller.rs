use std::collections::BTreeMap;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::base::ApiResponse;
use crate::services::user_doc::UserDocument;
use crate::AppState;

/// 邮箱注册表单
#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// 用户注册
/// @method post
/// @url /user/register
/// @body typeof RegisterForm
#[post("/user/register")]
pub async fn create_user(
    app_state: web::Data<AppState>,
    payload: web::Json<RegisterForm>,
) -> impl Responder {
    let new_user: RegisterForm = payload.into_inner();
    if let Some(user) = UserDocument::add_one(&app_state.db, new_user) {
        HttpResponse::Ok().json(ApiResponse::success(user))
    } else {
        HttpResponse::Ok().json(ApiResponse::error(5001, "注册用户失败"))
    }
}

/// 删除用户
/// @method delete
/// @url /user/{id}
#[delete("/user/{id}")]
pub async fn delete_user(
    app_state: web::Data<AppState>,
    payload: web::Path<String>,
) -> impl Responder {
    let text = payload.into_inner();
    if let Ok(id) = ObjectId::parse_str(text) {
        return if let Some(user) = UserDocument::delete_one(&app_state.db, id) {
            HttpResponse::Ok().json(ApiResponse::success(user))
        } else {
            HttpResponse::Ok().json(ApiResponse::error(5002, "删除用户失败"))
        };
    }
    return HttpResponse::Ok().json(ApiResponse::error(5004, "用户id错误"));
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserInfo {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub admin: Option<bool>,
    pub token: Option<String>,
}

/// 更新用户信息
/// @method put
/// @url /user/{id}
/// @body typeof UserInfo
#[put("/user/{id}")]
pub async fn update_user(
    app_state: web::Data<AppState>,
    payload: web::Path<String>,
    body: web::Json<UserInfo>,
) -> impl Responder {
    let text = payload.into_inner();
    if let Ok(id) = ObjectId::parse_str(text) {
        if let Some(username) = body.username.clone() {
            if username.len() < 5 {
                return HttpResponse::Ok().json(ApiResponse::error(5003, "用户名最少为5位"));
            }
        }
        if let Some(password) = body.password.clone() {
            if password.len() < 5 {
                return HttpResponse::Ok().json(ApiResponse::error(5003, "密码最少为5位"));
            }
        }
        if let Some(email) = body.email.clone() {
            if email.len() < 5 {
                return HttpResponse::Ok().json(ApiResponse::error(5003, "邮箱最少为5位"));
            }
        }
        if let Some(mut user) = UserDocument::find_by_id(&app_state.db, id) {
            if let Some(email) = body.email.clone() {
                user.email = email;
            }
            if let Some(user) = UserDocument::update_one(&app_state.db, &user) {
                return HttpResponse::Ok().json(ApiResponse::success(user));
            }
            return HttpResponse::Ok().json(ApiResponse::error(5002, "修改用户失败"));
        }
    }
    HttpResponse::Ok().json(ApiResponse::error(5004, "用户id错误"))
}

/// 分页查询
#[derive(Serialize, Deserialize, Debug)]
pub struct PageQuery {
    pub page: u64,
    pub page_size: u64,
}

/// 查询用户列表
/// @method get
/// @url /user
/// @query typeof PageQuery
#[get("/user")]
pub async fn find_user_list(
    app_state: web::Data<AppState>,
    payload: web::Query<PageQuery>,
) -> impl Responder {
    let page_query = payload.into_inner();
    if let Some(user) = UserDocument::find_many(&app_state.db, page_query) {
        HttpResponse::Ok().json(ApiResponse::success(user))
    } else {
        HttpResponse::Ok().json(ApiResponse::error(5003, "查找用户列表失败"))
    }
}

/// 查询单个用户
/// @method get
/// @url /user/{id}
#[get("/user/{id}")]
pub async fn find_user(
    app_state: web::Data<AppState>,
    payload: web::Path<String>,
) -> impl Responder {
    let text = payload.into_inner();
    if let Ok(id) = ObjectId::parse_str(text) {
        return if let Some(user) = UserDocument::find_by_id(&app_state.db, id) {
            HttpResponse::Ok().json(ApiResponse::success(user))
        } else {
            HttpResponse::Ok().json(ApiResponse::error(5002, "查找用户失败"))
        };
    }
    return HttpResponse::Ok().json(ApiResponse::error(5004, "用户id错误"));
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordLoginForm {
    username: String,
    password: String,
}

#[allow(dead_code)]
fn get_token_for_user(u: &PasswordLoginForm) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert(u.username.clone(), u.password.clone());
    let token_str = claims.sign_with_key(&key).unwrap();
    token_str
}

#[allow(dead_code)]
fn get_user_by_token(token: String) -> (String, String) {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();
    for (username, password) in claims {
        if username.len() >= 5 {
            return (username, password);
        }
    }
    return ("".to_string(), "".to_string());
}

/// 账号密码登录
#[post("/user/login")]
pub async fn do_login_by_password(
    app_state: web::Data<AppState>,
    payload: web::Form<PasswordLoginForm>,
) -> impl Responder {
    let form = payload.into_inner();
    if let Some(mut user) = UserDocument::find_by_username(&app_state.db, form.username.to_string())
    {
        println!("find_user {:?} , {:?}", user, form);
        if user.password == form.password {
            let token = get_token_for_user(&form);
            user.token = token.to_string();
            if let Some(_) = UserDocument::update_one(&app_state.db, &user) {
                user.password = "".to_string();
                return HttpResponse::Ok()
                    .header("set-cookie", format!("token={}", token))
                    .json(ApiResponse::success(user));
            }
            return HttpResponse::Ok().json(ApiResponse::error(5002, "登录失败，请稍后再试"));
        }
        return HttpResponse::Ok().json(ApiResponse::error(5002, "用户名或密码错误"));
    }
    HttpResponse::Ok().json(ApiResponse::error(5002, "用户名不存在"))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_eq() {
        let a = "123";
        let b = "123";
        let c = String::from("123");
        let d = String::from("123");
        assert_eq!(a, b);
        assert_eq!(c, d);
    }
}
