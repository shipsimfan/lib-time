use crate::{DateTime, TimeZone, Timestamp};

impl<T: TimeZone> Into<Timestamp<T>> for DateTime<T> {
    fn into(self) -> Timestamp<T> {
        todo!()
    }
}
