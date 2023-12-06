use crate::{
    model::{CustomerOrder, ModificationParams},
    schema::CustomerSchema,
    AppState,
};

use actix_web::{get, post, web, HttpResponse, Responder, Result};
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
                    return e.error_response();
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
                    return e.error_response();
                }
            }
        }
        Err(e) => {
            return e.error_response();
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

    let customer_query_result =
        sqlx::query(r#"INSERT INTO customers (email,phone, points) VALUES (?, ?, ?)"#)
            .bind(&customer.email)
            .bind(customer.phone)
            .bind(points)
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = customer_query_result {
        //TODO This error code is sqlite specific
        if err.contains("UNIQUE constraint failed") {
            // User exists, update user balance
            let balance_result = get_balance(&customer.email, &data.db).await;
            match balance_result {
                Ok(user_balance) => {
                    let total = points.round() as i64 + user_balance;
                    let set_result = set_points(&customer.email, total, &data.db).await;
                    match set_result {
                        Ok(_) => (),
                        Err(e) => {
                            return e.error_response();
                        }
                    }
                }
                Err(e) => {
                    return e.error_response();
                }
            };
        } else {
            //Some other error
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
        }
    }

    let order_query_result =
        sqlx::query(r#"INSERT INTO orders (id, paid, currency, percentage, customerEmail) VALUES (?, ?, ?, ?, ?)"#)
            .bind(&order.id)
            .bind(order.paid)
            .bind(&order.currency)
            .bind(percentage)
            .bind(&customer.email)
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());
    if let Err(err) = order_query_result {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    HttpResponse::Ok().json(points.to_string())
}
