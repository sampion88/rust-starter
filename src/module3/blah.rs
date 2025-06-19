use actix_web_lab::web as web_lab;
use actix_web::{web, App};
use actix_web::Either;
use rocket::response::Redirect;

fn get_redirect(dest: String) -> web_lab::Redirect {
    // ruleid: AIK_Rust-open-redirect
    web_lab::redirect("/".into(), dest)
}

fn safe_get_redirect(dest: String) -> web_lab::Redirect {
    // ok: AIK_Rust-open-redirect
    web_lab::redirect("/api", dest)
}

fn get_redirect2(dest: String) -> web_lab::Redirect {
    // ruleid: AIK_Rust-open-redirect
    web::redirect("/".into(), dest)
}

fn safe_get_redirect2(dest: String) -> web_lab::Redirect {
    // ok: AIK_Rust-open-redirect
    web::redirect("/api", dest)
}

async fn either_redirect(var: bool) -> impl Responder {
    if var {
        // ok: AIK_Rust-open-redirect
        Either::Left(Redirect::to("link_from_config"))
    } else {
        Redirect::to("/error")
    }
}

async fn redirect_(var: bool, request: HttpRequest) -> HttpResponse {
    if var {
    // ruleid: AIK_Rust-open-redirect
        Redirect::to(request.link)
            .respond_to(&request)
            .map_into_boxed_body()
    } else {
        Redirect::to("/error")
    }
}

async fn handler(path: String) -> impl Responder {
    // ruleid: AIK_Rust-open-redirect
    Redirect::to(format!("/api", path))
}

async fn handler2(path: String) -> impl Responder {
    // ruleid: AIK_Rust-open-redirect
    web::redirect("/", path)
}

async fn safe_handler2(path: String) -> impl Responder {
    // ok: AIK_Rust-open-redirect
    web::redirect("/api", path)
}

async fn handler3(path: String) -> impl Responder {
    // ruleid: AIK_Rust-open-redirect
    Redirect::to(path).permanent()
}

async fn handler4(path: String) -> impl Responder {
    // ruleid: AIK_Rust-open-redirect
    web::redirect("/", format!("www/google.com?search={}", path))
}

async fn safe_handler4(path: String) -> impl Responder {
    // ok: AIK_Rust-open-redirect
    web::redirect("/api", format!("/api?search={}", path))
}

async fn handler_scoped_safe(path: String) -> impl Responder {
    // ok: AIK_Rust-open-redirect
    Redirect::new("/api", path).permanent()
}

async fn handler_scoped_safe2(path: String) -> impl Responder {
    // ok: AIK_Rust-open-redirect
    web::scope("/api").service(web::redirect("/old", path))
}

async fn handler_scoped_safe2(path: String) -> impl Responder {
    // ok: AIK_Rust-open-redirect
    web::scope("/api").service(Redirect::to(path))
}

#[get("/hi/<name>/<age>")]
fn hi(name: String, age: u8) -> Redirect {
    // ok: AIK_Rust-open-redirect
    Redirect::to(uri!(hello: name, age))
}

#[get("/hi/<name>")]
fn hi(name: String) -> Redirect {
    // ok: AIK_Rust-open-redirect
    Redirect::to(format!("https://google.com/{}", name))
}

fn call(&self, request: ServiceRequest) -> Self::Future {
    let is_logged_in = false;

    if !is_logged_in && request.path() != "/login" {
        let (request, _pl) = request.into_parts();
        let response = HttpResponse::Found()
        // ruleid: AIK_Rust-open-redirect
            .insert_header((http::header::LOCATION, request.path()))
            .finish()

            .map_into_right_body();
			
        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
    }
    request.extensions_mut().insert::<Msg>(Msg("test message".to_owned()));
    let res = self.service.call(request);
    Box::pin(async move {
        res.await.map(ServiceResponse::map_into_left_body)
    })
}

pub fn redirect(
    from: String,
    to: String,
) -> Redirect {
    // ruleid: AIK_Rust-open-redirect
    web::redirect(from, to)
}

pub fn redirect_to_error(
    from: String,
    to: String,
) -> Redirect {
    // ok: AIK_Rust-open-redirect
    web::redirect(from, "/error")
}


pub fn redirect_safe(
    to: String,
) -> Redirect {
    let from_arg = to;
    if from_arg == "/users-handle" {
    // ok: AIK_Rust-open-redirect
        web::redirect("/api", to)
    } else {
        Redirect::to("/error")
    }
} 

#[post("/", data = "<data>")]
fn post_admin_login(
    data: Form<LoginForm>,
    cookies: &CookieJar<'_>,
    ip: ClientIp,
    referer: Referer,
) -> Result<Redirect, Flash<Redirect>> {
    let data = data.into_inner();

    if crate::ratelimit::check_limit_admin(&ip.ip).is_err() {
        // ok: AIK_Rust-open-redirect
        return Err(Flash::error(Redirect::to(admin_url(referer)), "Too many requests, try again later."));
    }

    // If the token is invalid, redirect to login page
    if !_validate_token(&data.token) {
        error!("Invalid admin token. IP: {}", ip.ip);
        // ok: AIK_Rust-open-redirect
        Err(Flash::error(Redirect::to(admin_url(referer)), "Invalid admin token, please try again."))
    }
}
