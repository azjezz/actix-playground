mod database;
mod form;
mod middleware;
mod security;
mod templating;
mod translation;

use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{cookie, error, web};
use actix_web::{App, HttpResponse, HttpServer, Responder, Result};
use security::SecurityToken;
use tarjama::locale::EnglishVariant;
use tarjama::locale::Locale;
use tera::{Context, Tera};

async fn index(
    tera: web::Data<Tera>,
    locale: Locale,
    token: SecurityToken,
) -> actix_web::Result<impl Responder> {
    if let SecurityToken::Authenticated { .. } = token {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/profile"))
            .finish());
    }

    let mut context = Context::new();
    context.insert("locale", &locale.to_string());
    let content = tera.render("index.html", &context).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

async fn login_ui(
    tera: web::Data<Tera>,
    locale: Locale,
    token: SecurityToken,
) -> Result<HttpResponse> {
    if let SecurityToken::Authenticated { .. } = token {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/profile"))
            .finish());
    }

    let mut context = Context::new();
    context.insert("locale", &locale.to_string());
    let content = tera.render("login.html", &context).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

async fn login(
    token: SecurityToken,
    pool: web::Data<database::Pool>,
    session: Session,
    form: web::Form<form::user::LoginFormData>,
) -> Result<impl Responder> {
    if let SecurityToken::Authenticated { .. } = token {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/profile"))
            .finish());
    }

    let email = form.email.clone();
    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        database::action::user::get_user_by_email(&mut conn, email.as_str())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::SeeOther()
                .append_header(("Location", "/login?error=invalid_credentials"))
                .finish());
        }
    };

    let is_valid = bcrypt::verify(form.password.as_str(), user.password.as_str())
        .map_err(error::ErrorInternalServerError)?;

    if !is_valid {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login?error=invalid_credentials"))
            .finish());
    }

    session
        .insert("user_id", user.id)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/profile"))
        .finish())
}

async fn register_ui(
    tera: web::Data<Tera>,
    locale: Locale,
    token: SecurityToken,
) -> Result<HttpResponse> {
    if let SecurityToken::Authenticated { .. } = token {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/profile"))
            .finish());
    }

    let mut context = Context::new();
    context.insert("locale", &locale.to_string());
    let content = tera.render("register.html", &context).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

async fn register(
    token: SecurityToken,
    pool: web::Data<database::Pool>,
    session: Session,
    form: web::Form<form::user::RegisterFormData>,
) -> Result<impl Responder> {
    if let SecurityToken::Authenticated { .. } = token {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/profile"))
            .finish());
    }

    let hash = bcrypt::hash(form.password.as_str(), bcrypt::DEFAULT_COST)
        .map_err(error::ErrorInternalServerError)?;

    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        database::action::user::insert_new_user(
            &mut conn,
            form.username.as_str(),
            form.email.as_str(),
            hash.as_str(),
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    session
        .insert("user_id", user.id)
        .map_err(error::ErrorInternalServerError)?;

    return Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/profile"))
        .finish());
}

async fn profile(
    tera: web::Data<Tera>,
    locale: Locale,
    token: SecurityToken,
) -> actix_web::Result<impl Responder> {
    if !token.is_authenticated() {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish());
    }

    let user = token.user().unwrap();

    let mut context = Context::new();
    context.insert("user", &user);
    context.insert("locale", &locale.to_string());
    let content = tera.render("profile.html", &context).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}

async fn logout(token: SecurityToken, session: Session) -> actix_web::Result<impl Responder> {
    if let SecurityToken::Authenticated { .. } = token {
        session.remove("user_id");
    }

    return Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/login?logout=1"))
        .finish());
}

async fn default_handler(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let content = tera.render("error/404.html", &Context::new()).unwrap();

    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body(content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = database::initialize_db_pool();
    let translator = translation::initialize_translator();

    let mut engine = templating::initialize_engine();
    engine.register_filter(
        "trans",
        templating::filter::TranslatorFilter::new(translator.clone()),
    );

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(engine.clone()))
            .wrap(middleware::user::UserDataMiddleware)
            .wrap(tarjama::actix::TranslatorMiddleware::new(
                translator.clone(),
                Locale::English(EnglishVariant::Default),
            ))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .default_service(web::route().to(default_handler))
            .route("/", web::get().to(index))
            .service(
                web::resource("/login")
                    .route(web::get().to(login_ui))
                    .route(web::post().to(login)),
            )
            .service(
                web::resource("/register")
                    .route(web::get().to(register_ui))
                    .route(web::post().to(register)),
            )
            .service(web::resource("/profile").route(web::get().to(profile)))
            .service(web::resource("/logout").route(web::get().to(logout)))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(num_cpus::get() * 2)
    .run()
    .await
}
