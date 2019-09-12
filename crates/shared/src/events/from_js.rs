use serde::Deserialize;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::convert::TryFrom;

/**
 * It's simplest for core to match on a single event_type 
 * rather than try to deserialize against all possibilities
 * 
 * getting event_type automatically means splitting it since
 * `FromPrimitive` cannot be impled for enums that hold data 
 *  
 */

//the order must match typescript!
#[derive(FromPrimitive)]
#[repr(u32)]
pub enum CoreEventIndex {
    ToggleAudio,
    SetSpeed,
    WindowSize,
    RendererLoaded,
    AudioLoaded,
    Started
}


//Let's us get a CoreEvent from the number which is sent from JS
impl TryFrom<u32> for CoreEventIndex {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("CoreEvent: outside of range!")
    }
}

