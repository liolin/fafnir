use diesel::r2d2;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use std::env;
use std::thread;

fn main() {

    dotenv().ok();
    let manager: r2d2::ConnectionManager<PgConnection> = r2d2::ConnectionManager::new(env::var("DATABASE_URL").unwrap());
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    for _ in 0..20 {
        let pool = pool.clone();
        thread::spawn(move || {
            let _conn = pool.get().unwrap();
            // use the connection
            // it will be returned to the pool when it falls out of scope.
        });
    }
}
