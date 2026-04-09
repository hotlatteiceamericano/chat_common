use mongodb::bson::oid::ObjectId;
use ratatui::{text::Line, widgets::ListItem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[cfg_attr(feature = "mongodb", serde(rename = "_id",))]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    id: Option<ObjectId>,
    display_name: String,
    email: String,
}

impl User {
    pub fn new(display_name: &str, email: String) -> Self {
        Self {
            id: None,
            display_name: display_name.to_string(),
            email,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
            .expect("expecting MongoDB to auto generate user id upon insertion")
            .to_hex()
            .parse::<u32>()
            .expect("expect user id to be numeric")
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
