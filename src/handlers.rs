#[path = "models.rs"]
mod models;

pub mod px_generators{

    use actix_web::{
        web,
        Error,
        HttpResponse
    };

///////////////////////////////////////////////////////////////////////////////////////////////////
// DYNAMIC PX2 GEN FUNCTION

    pub async fn genpx2 (
        client_req: web::Json<Px2ClientReq>,
    ) -> Result<HttpResponse, Error> {
        let client_req_inf: Px2ClientReq = client_req.into_inner();

        // DYNAMIC PXAPPID!
        // so this sets the px appid based on the site in the req
        if !client_req_inf.site {
            Ok(HttpResponse::BadRequest())
        } else {
            let px2_appid: String = match client_req_inf.site {
                // add more sites here
                "walmart" => "PXu6b0qd2S",
                "hibbet" => "PXAJDckzHD",
                "solebox" => "PXuR63h57Z",
                _ =>(),
            };

            //just responding to request with the px appid for now
            Ok(HttpResponse::Ok().json(px2_appid))
        }
    }

///////////////////////////////////////////////////////////////////////////////////////////////////
// DYNAMIC PX3 GEN FUNCTION

    pub async fn genpx3 (
        client_req: web::Json<Px3ClientReq>,
    ) -> Result<HttpResponse, Error> {
        // same tesing junk...
        let client_req_inf: Px3ClientReq = client_req.into_inner();
        Ok(HttpResponse::Ok().json(client_req_inf))
    }
}
