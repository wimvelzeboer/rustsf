use chrono::{Utc};

pub type Id18 = String;

pub type Datetime = chrono::DateTime<Utc>;

pub trait SObject {
    fn id(&self) -> Id18;

    fn set_id(&mut self, id: Id18) -> &mut Self;
}