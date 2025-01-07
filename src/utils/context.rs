use diesel::internal::derives::multiconnection::chrono::{DateTime, Utc};
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct RequestContext{
    pub timestamp: DateTime<Utc>,
    pub user: Option<RequestUser>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestUser{
    pub login: String,
}

