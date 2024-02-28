use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};

struct SecurityAddons;

impl Modify for SecurityAddons {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
paths(
crate::routes::auth_routes::AuthRoutes::login,
crate::routes::user_routes::UserRoutes::create,
crate::routes::user_routes::UserRoutes::get,
crate::routes::user_routes::UserRoutes::update,
crate::routes::user_routes::UserRoutes::delete,
crate::routes::school_routes::SchoolRoutes::create,
crate::routes::school_routes::SchoolRoutes::get,
crate::routes::school_routes::SchoolRoutes::update,
crate::routes::school_routes::SchoolRoutes::delete
),
componets(
schemas(
utpoia::TupleUnit,
crate::schemas::auth_schema::LoginRequest,
crate::schemas::auth_schema::LoginResponse,
crate::schemas::user_schemas::UserCreate,
crate::schemas::user_schemas::UserResponse,
crate::schemas::user_schemas::UserUpdate,
crate::schemas::school_schemas::SchoolCreate,
crate::schemas::school_schemas::SchoolResponse,
crate::schemas::school_schemas::SchoolUpdate
)
)
tags(name = "User Authentication Service", description = "User Authentication Service API in \
    Rust"),
modifiers(
& SecurityAddons
)
)]
pub struct ApiDoc;
