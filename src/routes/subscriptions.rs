use crate::domain::{NewSubscriber, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, PgPool};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name="Adding a new subscriber", 
    skip(form, pool),
    fields(email=%form.email, name=%form.name)
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let name =
        SubscriberName::parse(form.0.name).map_err(|_| HttpResponse::BadRequest().finish())?;
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: name,
    };
    insert_subscriber(&pool, &new_subscriber)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    tracing::info!("New subscriber details have been saved");
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Saving a new subscribers", skip(pool, new_subscriber))]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
