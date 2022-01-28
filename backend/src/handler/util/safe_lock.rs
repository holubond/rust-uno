use actix_web::{web, HttpResponse};
use std::sync::{Mutex, MutexGuard};

use super::response::ErrMsg;

pub fn safe_lock<'a, T>(obj: &'a web::Data<Mutex<T>>) -> Result<MutexGuard<'a, T>, HttpResponse> {
    match obj.lock() {
        Err(_) => {
            Err(HttpResponse::InternalServerError().json(
                ErrMsg::new_from_scratch("Cannot obtain lock on repo"))
            )
        }
        Ok(x) => Ok(x),
    }
}
