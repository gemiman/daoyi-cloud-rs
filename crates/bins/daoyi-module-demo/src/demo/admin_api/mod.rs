use salvo::prelude::*;

pub mod auth;
pub mod user;

pub fn create_router() -> Router {
    let auth_router = auth::create_router();
    let user_router = user::create_router();

    Router::new().push(
        Router::with_path("/demo").push(
            Router::new()
                .push(Router::with_path("/users").push(user_router))
                .push(Router::with_path("/auth").push(auth_router)),
        ),
    )
}
