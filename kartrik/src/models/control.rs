use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Control {
    #[field(validate = len(3..30))]
    pub(crate) title: String,
    #[field(validate = len(3..300))]
    pub(crate) description: String,
}
