use std::fmt;
use std::fmt::{Display, Formatter};
use std::task::{Context, Poll};
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, ResponseError};
use actix_web::body::{BodySize, BoxBody, MessageBody};
use actix_web::http::StatusCode;
use utoipa::ToSchema;
use serde::Deserialize;
use serde::Serialize;
use crate::error::AppError;

pub type AppResult = Result<MessageResponse, AppError>;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
    pub data :Option<serde_json::Value>,
    pub code : u16
}


impl  MessageResponse {
    pub fn new( data: Option<serde_json::Value>) -> Self {
        Self {
            message: "success".into(),
            data,
            code:StatusCode::OK.as_u16()
        }
    }
}
impl Responder for MessageResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
