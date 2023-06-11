use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    #[serde(default)]
    n: Option<u64>,
    #[serde(default)]
    m: Option<u64>,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if let (Some(n), Some(m)) = (form.n, form.m) {
        if n == 0 || m == 0 {
            return HttpResponse::BadRequest()
                .content_type("text/html")
                .body("Computing the GCD with zero is boring");
        }
        let response = format!("{} and {} is <b>{}</b>\n", n, m, gcd(n, m));
        HttpResponse::Ok()
            .content_type("text/html")
            .body(response)
    } else {
        HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Invalid parameters")
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")
    .expect("Failed to bind address")
    .run()
    .expect("Failed to run the server");

    println!("Serving on http://localhost:3000....");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#,
        )
}
