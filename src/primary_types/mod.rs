use chrono::{Utc};

pub type Id18 = String;

pub type Datetime = chrono::DateTime<Utc>;

pub trait SObject {
    fn id(&self) -> Option<&str>;

    fn get_sobject_type(&self) -> &str;

    fn set_id(&mut self, id: Option<&str>) -> &mut Self;

    fn set_owner_id(&mut self, id: Option<&str>) -> &mut Self;

    fn get_owner_id(&self) -> Option<&str>;
}