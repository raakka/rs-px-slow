pub mod models {
    use serde::{
        Serialize,
        Deserialize,
    };

///////////////////////////////////////////////////////////////////////////////////////////////////
//PX2

    //TODO(XVI): Make this an Enum so you don't have to use multiple objs

    // this is the req we expect from the client when genning PX2 
    #[derive(Serialize, Deserialize)]
    pub struct Px2ClientReq {
        pub site: String,
        pub pxnum: String,
    } 

    // this is the res we send the client when genning for PX2
    // this is also what we expect back from the PX2 req
    #[derive(Serialize, Deserialize)]
    pub struct Px2ClientRes {
        // lol this is a bad way of getting past typechecking but whatever :/
        pub t: String,
        pub d: Map<String, Value>,
    }
    
///////////////////////////////////////////////////////////////////////////////////////////////////
// PX3

    // TODO(XVI): Make the PX3 reqs correct... these are just BS
    // this is the req we expect from the client when genning PX3 
    #[derive(Serialize, Deserialize)]
    pub struct Px3ClientReq {
        pub site: String,
        pub pxnum: String,
        pub seq: i32,
        pub uuid: String,
    }

    // this is the res we send the client when genning for PX3
    #[derive(Serialize, Deserialize)]
    pub struct Px3ClientRes {
        pub site: String,
        pub pxnum: String,
        pub seq: i32,
        pub uuid: String,
    }
}
