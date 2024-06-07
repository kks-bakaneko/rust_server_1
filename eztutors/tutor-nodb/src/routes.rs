use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .route("/health", web::get().to(health_check_handler));
}
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/courses")
        .route("/",web::post().to(new_course))
        .route("/{user_id}", web::get().to(get_courses_for_tutor))
        .route("/{user_id}/{course_id}", web::get().to(get_course_detail)), //get,post는 ().to(function)으로 제대로 할것
    ); //아 제발 괄호좀 이새끼야 아아아아아아앍 
}