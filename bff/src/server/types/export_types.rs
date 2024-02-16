mod user;
mod session;
pub use user::User;
pub use session::Session;

// реэкспорт клиентскому коду, типы общие для бекенда и фронтенда
pub use crate::server::types::db_interaction_types::{Role, RoleType};