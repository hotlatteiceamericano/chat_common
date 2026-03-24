use ratatui::{text::Line, widgets::ListItem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[cfg_attr(
        feature = "mongodb",
        serde(
            serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string",
            rename = "_id",
        )
    )]
    id: u32,
    display_name: String,
    email: String,
}

impl User {
    pub fn new(id: u32, display_name: &str, email: String) -> Self {
        Self {
            id,
            display_name: display_name.to_string(),
            email,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn display_name(&self) -> &str {
        self.display_name.as_str()
    }
}

impl<'a> From<&'a User> for ListItem<'a> {
    fn from(value: &'a User) -> ListItem<'a> {
        ListItem::new(Line::from(format!(
            "{}({})",
            value.display_name(),
            value.id()
        )))
    }
}
