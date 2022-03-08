// Original code (MIT license):
// https://github.com/petertrotman/actix-web-middleware-redirect-https
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    http,
    Error,
    HttpResponse,
};
use futures::future::{ok, Either, Ready};


#[derive(Default, Clone)]
pub struct RedirectHTTPS {
    replacements: Vec<(String, String)>,
}

impl RedirectHTTPS {
    pub fn with_replacements(replacements: &[(String, String)]) -> Self {
        RedirectHTTPS {
            replacements: replacements.to_vec(),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for RedirectHTTPS
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectHTTPSService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectHTTPSService {
            service,
            replacements: self.replacements.clone(),
        })
    }
}

pub struct RedirectHTTPSService<S> {
    service: S,
    replacements: Vec<(String, String)>,
}

impl<S> Service<ServiceRequest> for RedirectHTTPSService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future =
        Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    actix_web::dev::forward_ready!(service);

    fn call(
        &self,
        req: ServiceRequest,
    ) -> Self::Future {

        if req.connection_info().scheme() == "https" {

            Either::Left(self.service.call(req))

        } else {

            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();

            let mut url = format!("https://{}{}", host, uri);
            for (s1, s2) in self.replacements.iter() {
                url = url.replace(s1, s2);
            }

            Either::Right(
                ok(
                    req.into_response(
                        HttpResponse::MovedPermanently()
                            .append_header((http::header::LOCATION, url))
                            .finish()
                    ).map_into_boxed_body()
                )
            )
        }

    }
}

