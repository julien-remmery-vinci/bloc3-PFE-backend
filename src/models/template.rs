use serde::Serialize;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Template {
    pub template_id: i32,
    pub value: String
}