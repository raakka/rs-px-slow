#[path = "models.rs"]
mod models;

pub mod px_generators{
    use super::models::models::{
        Px2ClientReq,
        Px2ClientRes,
        Px3ClientReq,
        Px3ClientRes
    };

    use actix_web::{ 
        web,
        Error,
        HttpResponse
    };
   
    pub async fn genpx2 (
        client_req: web::Json<Px2ClientReq>,
    ) -> Result<HttpResponse, Error> {
        // this is just testing
        let client_req_inf: Px2ClientReq = client_req.into_inner();
        // we are just returing the request given to the ws
        Ok(HttpResponse::Ok().json(client_req_inf))
    }

    pub async fn genpx3 (
        client_req: web::Json<Px3ClientReq>,
    ) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().json(client_req))
    } 
}
