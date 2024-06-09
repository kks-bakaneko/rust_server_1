use crate::dbaccess::tutor::{self, *};
use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, UpdateTutor};
use crate::state::AppState;

use actix_web::{web, App, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>
) -> Result<HttpResponse, EzyTutorError> {
    let tutor = tutor_id.into_inner();
    get_tutor_details_db(&app_state.db, tutor)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    app_state: web::Data<AppState>,
    new_tutor: web::Json<NewTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    //let tutor = tutor_id.into_inner();
    let tutor_detail = NewTutor::from(new_tutor);
    post_new_tutor_db(&app_state.db,  tutor_detail)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor = tutor_id.into_inner();
    let tutor_detail = UpdateTutor::from(update_tutor);
    update_tutor_details_db(&app_state.db, tutor, tutor_detail)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor = tutor_id.into_inner();
    delete_tutor_db(&app_state.db, tutor)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}