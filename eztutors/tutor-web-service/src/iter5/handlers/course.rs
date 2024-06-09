use crate::dbaccess::course::*;
use crate::models::course::*;
use crate::state::AppState;
use crate::errors::EzyTutorError;
use actix_web::{web, HttpResponse};


pub async fn get_course_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tuple = _params.into_inner();
    let tutor_id: i32 = i32::try_from(tuple.0).unwrap();
    get_courses_for_tutor_db(&_app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path::<(i32,i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tuple = params.into_inner();
    let tutor_id:i32 = i32::try_from(tuple.0).unwrap();
    let course_id:i32 = i32::try_from(tuple.1).unwrap();
    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
    
}

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state : web::Data<AppState>,
    path : web::Path<(i32, i32)>
) -> Result<HttpResponse, EzyTutorError> {
    let tuple = path.into_inner();
    let tutor_id = i32::try_from(tuple.0).unwrap();
    let course_id = i32::try_from(tuple.1).unwrap();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(
    app_state : web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32,i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tuple = path.into_inner();
    let tutor_id = i32::try_from(tuple.0).unwrap();
    let course_id = i32::try_from(tuple.1).unwrap();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_cuourses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_course_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_course_details_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,2));
        let resp = get_course_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK)
    }
    #[actix_rt::test]
    async fn get_course_details_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,21));
        let resp = get_course_details(app_state, params).await; // unwrap으로 처리하면 resp가 이미 결과 처리된 상태로 들어가지... 에라이
        match resp {
            Ok(_) => println!("Something went wrong!!"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }
    #[ignore]
    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_course_msg = CreateCourse {
            tutor_id:4,
            course_name:"This is the next course".into(),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price:None,
            course_duration: None,
            course_language: Some("Korean".into()),
            course_structure: None,
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let update_course_msg = UpdateCourse {
            course_name:Some("This is the next course".into()),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Intermediate".into()),
            course_price:None,
            course_duration: None,
            course_language: Some("English".into()),
            course_structure: None,
        };
        let params: web::Path<(i32,i32)> = web::Path::from((1,1));
        let update_param = web::Json(update_course_msg);
        let resp = update_course_details(app_state, update_param,params ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[ignore]
    #[actix_rt::test]
    async fn delete_course_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,5));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK)
    }
    
    #[actix_rt::test]
    async fn delete_course_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,21));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }   
}