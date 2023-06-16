use super::handler::{
    Handler,
    PageNotFoundHanlder,
    StaticPageHandler,
    WebServiceHandler,
};
use http::{
    httprequest,
    httprequest::HttpRequest,
    httpresponse::HttpResponse,
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        println!("HttpRequest --- {:#?}", req);
        match req.method {
            httprequest::Method::Get => match &&req.resource {  
                httprequest::Resource::Path(s) => {
                    println!("path --- {:?}", s);
                    let route:Vec<&str> = s.split("/").collect();
                    println!("route --- {:?}", route);
                    match route[1] {
                        "api" => {
                            let resp:HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp:HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHanlder::handle(&req);
                let _ = resp.send_response(stream);
           } 
        }
    }
}
