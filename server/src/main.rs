use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::{web};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::io;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod controllers;
mod models;
mod routes;
mod services;
mod database;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::user_controller::get_all_users,
        controllers::user_controller::get_user_by_id,
        controllers::user_controller::create_user,
        controllers::user_controller::update_user,
        controllers::user_controller::delete_user,
        controllers::income_controller::get_all_incomes,
        controllers::income_controller::get_incomes_by_user_id,
        controllers::income_controller::create_income,
        controllers::income_controller::update_income,
        controllers::income_controller::delete_income,
        controllers::expense_controller::get_all_expenses,
        controllers::expense_controller::get_expenses_by_user_id,
        controllers::expense_controller::create_expense,
        controllers::expense_controller::update_expense,
        controllers::expense_controller::delete_expense,
    ),
    components(
        schemas(
            models::user::User,
            models::user::NewUser,
            models::user::UpdateUser,
            models::user::UserWithIncomes,
            models::income::Income,
            models::income::NewIncome,
            models::income::UpdateIncome,
            models::income::IncomeWithUser,
            models::expense::Expense,
            models::expense::NewExpense,
            models::expense::UpdateExpense
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "incomes", description = "Income management endpoints"),
        (name = "expenses", description = "Expense management endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    let database_url = config::get_database_url();
    let server_url = config::get_server_url();
    log::info!("Starting server at: {}", server_url);
    log::info!("Swagger UI available at: {}/swagger-ui/", server_url);

    let pool = database::db_connection::create_connection_pool(&database_url);
    let mut conn = database::db_connection::get_connection(&pool)
        .expect("Failed to get connection from pool");
    database::db_migrations::run_migrations(&mut conn);

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        // Configure custom logger
        let logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T");

        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec!["content-type", "authorization"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(logger)
            .app_data(config::errors::json_error_handler())
            .configure(routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
