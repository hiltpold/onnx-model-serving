use super::error::AppError;
use super::model::{Health, ImageRequest, ImageResponse};
use super::utils;
use super::yolo;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Responder};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use image::{self, ImageFormat};
use std::env;
use validator::Validate;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(health)
        .service(objectdetection);

    conf.service(scope);
}

#[post("/objectdetection")]
async fn objectdetection(req: web::Json<ImageRequest>) -> Result<impl Responder, AppError> {
    let is_valid = req.validate().map_err(|err| AppError::from(err));
    let mut model_path = env::current_dir().unwrap();

    match is_valid {
        Ok(_) => {
            let image_format = ImageFormat::from_extension(req.image_format.to_owned()).unwrap();
            model_path.push("model");
            model_path.push(req.model_name.to_owned());
            let image_buffer = BASE64.decode(&req.image).unwrap();
            let image = image::load_from_memory_with_format(&image_buffer, image_format).unwrap();
            // get classes the model is trained on
            let classes = utils::get_classes();
            // detect object(s) in the image using the onnx model
            let inferred_image = match yolo::detect(model_path, &classes, image) {
                Ok(result) => result,
                Err(error) => {
                    // TODO: correct error handling
                    panic!("A problem occured during inference: {:?}", error)
                }
            };

            Ok(HttpResponse::Ok().json(ImageResponse {
                result_image: BASE64.encode(&inferred_image.result_image),
                bounding_boxes: inferred_image.bounding_boxes,
            }))
        }
        Err(err) => Err(err),
    }
}

#[get("/heartbeat")]
async fn health() -> impl Responder {
    let health = Health {
        status: "running".to_owned(),
    };
    let response = serde_json::to_string(&health).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
