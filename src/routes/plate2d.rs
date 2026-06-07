use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreatePlate2d {
    name:      String,
    width_mm:  f32,
    height_mm: f32,
    t_hot:     f32,
    t_amb:     f32,
}

#[derive(Serialize, ToSchema)]
pub struct Plate2d {
    id:        String,
    name:      String,
    width_mm:  f32,
    height_mm: f32,
    t_hot:     f32,
    t_amb:     f32,
}

#[utoipa::path(
    post,
    path = "/plate2d",
    request_body = CreatePlate2d,
    responses(
        (status = 200, description = "Plate created", body = Plate2d)
    )
)]
#[post("/plate2d")]
pub(crate) async fn create_plate2d(body: web::Json<CreatePlate2d>) -> impl Responder {
    let plate = Plate2d {
        id:        Uuid::new_v4().to_string(),
        name:      body.name.clone(),
        width_mm:  body.width_mm,
        height_mm: body.height_mm,
        t_hot:     body.t_hot,
        t_amb:     body.t_amb,
    };
    HttpResponse::Ok().json(plate)
}