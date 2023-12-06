use crate::{
    model::{Customer, CustomerOrder, ModificationParams, Order, RewardParams},
    schema::{CustomerSchema, OrderSchema},
    AppState,
};

use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Sqlite};

//TODO: abstract sqlite
pub async fn get_balance(user: &String, database: &Pool<Sqlite>) -> Result<i64> {
    let result = sqlx::query_as!(
        CustomerSchema,
        r#"SELECT * FROM customers WHERE email = ?"#,
        user
    )
    .fetch_one(database)
    .await;
    match result {
        Ok(user) => {
            return Ok(user.points);
        }

        Err(sqlx::Error::RowNotFound) => {
            return Err(actix_web::error::ErrorNotFound(format!(
                "Could not find user {:?}",
                user.to_string()
            )));
        }
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };
}

pub async fn set_points(user: &String, points: i64, database: &Pool<Sqlite>) -> Result<i64> {
    let result = sqlx::query_as!(
        CustomerSchema,
        r#"UPDATE customers SET points = ? WHERE email = ?"#,
        points,
        user
    )
    .fetch_optional(database)
    .await;
    match result {
        Ok(_) => {
            return Ok(points);
        }

        Err(sqlx::Error::RowNotFound) => {
            return Err(actix_web::error::ErrorNotFound(
                "Could not find user {user}",
            ));
        }
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };
}

#[get("user/{user}/balance")]
pub async fn get_user_balance(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user = path.into_inner();
    let result = get_balance(&user, &data.db).await;
    match result {
        Ok(balance) => {
            let user_response = json!({"status": "success","data": json!({
                "email": user,
                "balance": balance,
            })});
            return HttpResponse::Ok().json(user_response);
        }

        Err(e) => {
            return e.error_response();
        }
    };
}

#[post("/user/{user}/add")]
pub async fn post_add_points(
    path: web::Path<String>,
    data: web::Data<AppState>,
    json: web::Json<ModificationParams>,
) -> impl Responder {
    let user = path.into_inner();
    let amount = json.amount;

    let result = get_balance(&user, &data.db).await;
    match result {
        Ok(user_balance) => {
            let points = amount + user_balance;
            let set_result = set_points(&user, points, &data.db).await;
            match set_result {
                Ok(points) => {
                    return HttpResponse::Ok().json(points);
                }
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(json!({"status": "error","message": format!("{:?}", e)}));
                }
            }
        }
        Err(e) => {
            return e.error_response();
        }
    };
}

#[post("/user/{user}/sub")]
pub async fn post_sub_points(
    path: web::Path<String>,
    data: web::Data<AppState>,
    json: web::Json<ModificationParams>,
) -> impl Responder {
    let user = path.into_inner();
    let amount = json.amount;

    let result = get_balance(&user, &data.db).await;

    match result {
        Ok(user_balance) => {
            if user_balance < amount {
                return HttpResponse::BadRequest()
                        .json(json!({"status": "error","message": "Amount to subtract was more than available"}));
            }
            let points = user_balance - amount;
            let set_result = set_points(&user, points, &data.db).await;
            match set_result {
                Ok(points) => {
                    return HttpResponse::Ok().json(points);
                }
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(json!({"status": "error","message": format!("{:?}", e)}));
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}));
        }
    };
}

#[post("/orders/new")]
pub async fn post_new_order(
    data: web::Data<AppState>,
    customer_order: web::Json<CustomerOrder>,
) -> impl Responder {
    let customer = &customer_order.customer;
    let order = &customer_order.order;

    let mut percentage = 0.01;
    if let Some(reward) = &customer_order.reward_params {
        percentage = reward.amount;
    }
    let points = percentage * customer_order.order.paid as f64;
    
    let query_result =
        sqlx::query(r#"INSERT INTO customer (email,phone, points) VALUES (?, ?, ?)"#)
            .bind(&customer.email)
            .bind(customer.phone)
            .bind(points)
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            //TODO Add balance to user
            return HttpResponse::BadRequest().json(
            serde_json::json!({"status": "fail","message": "Note with that title already exists"}),
        );
        }

        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }
    //TODO add order
    HttpResponse::Ok().json(points.to_string())
}
