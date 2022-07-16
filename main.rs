extern crate simple_proxy;

mod middlewares;

use middlewares::auth::Auth;
use simple_proxy::middlewares::{Cors, Health, Logger, Router};
use simple_proxy::SimpleProxy;

fn main() {
    let config = config::Config::new();
    let mut proxy = SimpleProxy::new(config.port, config.environment);
    
    // Middlewares
    let auth = Auth::new(config.clone());
    let health = Health::new("/health", "OK !");
    let router = Router::new(config);
    let logger = Logger::new();
    let cors = Cors::new(
        "*",
        "GET, POST, PATCH, DELETE, OPTIONS",
        "Content-Type, Accept, Authorization, X-Requested-Ids, X-Tenant",
    );

    // Order matters
    proxy.add_middleware(Box::new(logger));
    proxy.add_middleware(Box::new(cors));
    proxy.add_middleware(Box::new(health));
    proxy.add_middleware(Box::new(router));
    proxy.add_middleware(Box::new(auth));

    // Start proxy
    proxy.run();
}
