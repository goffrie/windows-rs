#![allow(clippy::many_single_char_names)]

use super::*;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct GUID(pub u32, pub u16, pub u16, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl GUID {
    pub fn from_args(args: &[(String, Value)]) -> Option<Self> {
        if args.len() < 11 {
            return None;
        }
        let a = match &args[0].1 {
            Value::U32(value) => *value,
           _ => return None,
        };
        let b = match &args[1].1 {
            Value::U16(value) => *value,
           _ => return None,
        };
        let c = match &args[2].1 {
            Value::U16(value) => *value,
           _ => return None,
        };
        let d = match &args[3].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let e = match &args[4].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let f = match &args[5].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let g = match &args[6].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let h = match &args[7].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let i = match &args[8].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let j = match &args[9].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        let k = match &args[10].1 {
            Value::U8(value) => *value,
           _ => return None,
        };
        Some(Self(a,b,c,d,e,f,g,h,i,j,k))
    }
}

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
