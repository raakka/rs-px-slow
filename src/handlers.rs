#[path = "models.rs"]
mod models;

pub mod px_generators{
    
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
        
        // okay are we doing things now?
        // this is mad invalid rn 
        // TODO(XVI): fix this bs bro??!!
        let px2_result: web::Json<Px2ClientRes> = match client_req_inf.site {
            // add more sites here
            "walmart" => ok(),
            "hibbet" => ok(),
            "solebox" => ok(),
        }

        Ok(HttpResponse::Ok().json(px2_result.into_inner()))
    }

    pub async fn genpx3 (
        client_req: web::Json<Px3ClientReq>,
    ) -> Result<HttpResponse, Error> {
        // same tesing junk...
        let client_req_inf: Px3ClientReq = client_req.into_inner();
        Ok(HttpResponse::Ok().json(client_req_inf))
    } 
}
