use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use validator::ValidationError;

lazy_static! {
    static ref RE_FORMAT: Regex = Regex::new(r"^(jpg|jpeg|png)$").unwrap();
    static ref RE_MODEL: Regex = Regex::new(r"^(militaryaircrafts.onnx)$").unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Health {
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ImageRequest {
    #[validate(
        regex(
            path = "RE_MODEL",
            message = "Image format must be one of (militaryaircrafts.onnx)"
        ),
        custom(function = "model_exists", message = "Model does not exist")
    )]
    pub model_name: String,
    #[validate(length(min = 1))]
    pub image_name: String,
    #[validate(regex(
        path = "RE_FORMAT",
        message = "Image format must be one of (jpg, jpeg or png)"
    ))]
    pub image_format: String,
    #[validate(custom(function = "is_base64", message = "String is not base64 encoded"))]
    pub image: String,
}

fn is_base64(base64image: &str) -> Result<(), ValidationError> {
    let image_buffer = BASE64.decode(base64image);
    let res = match image_buffer {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Password Validation Failed")),
    };
    return res;
}

fn model_exists(model_name: &str) -> Result<(), ValidationError> {
    let mut model_path = env::current_dir().unwrap(); //.to_str().unwrap().to_string();
    model_path.push("model");
    model_path.push(model_name.to_owned());

    let res = match model_path.exists() {
        true => Ok(()),
        false => Err(ValidationError::new("Model does not exist")),
    };
    return res;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResponse {
    pub image_name: String,
    pub result_image: String,
    pub bounding_boxes: Vec<i32>,
}

// Implement Responder Trait for Image
impl Responder for ImageResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}
