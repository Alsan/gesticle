#[macro_use]
extern crate log;

pub mod configuration;
pub mod events;
pub mod gestures;
pub mod dbus;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
