mod app;
mod error;

use std::env;

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    let sys = actix::System::new("super-match");

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    app::run(bind_address).expect("App run failed");

    let _ = sys.run();
}
