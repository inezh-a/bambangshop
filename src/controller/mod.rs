pub mod product;
pub mod notification;

use rocket::{fairing::AdHoc, route};

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/product", routes![product::create, product::list, product::read, product::delete])
            .mount("/notification", route![])
    });
}
