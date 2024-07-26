use crate::{
    bindings::{
        component::grafbase::types::{Context, Error, Headers},
        exports::component::grafbase::gateway_request,
    },
    init_logging, Component,
};

impl gateway_request::Guest for Component {
    fn on_gateway_request(context: Context, headers: Headers) -> Result<(), Error> {
        init_logging();

        if let Some(id) = headers.get("x-current-user-id") {
            tracing::info!("Setting current user to {id}");
            context.set("current-user-id", &id);
        }
        if let Some(role) = headers.get("x-role") {
            context.set("role", &role);
        }

        Ok(())
    }
}
