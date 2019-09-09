use serde::Deserialize;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::convert::TryFrom;

/**
 * `FromPrimitive` can be applied only to unitary enums and newtypes,
 * therefore we need to split the event type vs. event data
 */

//All the core event types:
#[derive(FromPrimitive)]
#[repr(u32)]
pub enum CoreEvent {
    ToggleAudio,
    SetSpeed,
    WindowSize 
}

//All the core event data:
#[derive(Deserialize)]
pub struct Speed(pub f64);

//Let's us get a CoreEvent from the number which is sent from JS
impl TryFrom<u32> for CoreEvent {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("CoreEvent: outside of range!")
    }
}
