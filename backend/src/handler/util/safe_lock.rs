use std::sync::{Mutex, MutexGuard, Arc};
use actix_web::{HttpResponse, web};

use super::response::ErrorMessage;

pub fn safe_lock<'a, T>(obj: &'a web::Data<Arc<Mutex<T>>>) -> Result<MutexGuard<'a, T>, HttpResponse> {
    match obj.lock() {
        Err(_) => Err(
                        HttpResponse::InternalServerError().json(
                            ErrorMessage::new("Cannot obtain lock on repo")
                        )
                    ),
        Ok(x) => Ok(x)
    }
}