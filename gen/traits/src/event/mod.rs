use crate::ty::EventItem;

pub trait Event {
    fn register_event(&self)-> Vec<EventItem<Self>> where Self: Sized;
}