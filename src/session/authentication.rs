
use super::{controller};
use ntex_session::Session;

pub fn authenticate(session : Session) -> bool {
    return controller::check_token(session);
} 
