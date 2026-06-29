use alloc::boxed::Box;
use alloc::string::String;
use alloc::layout::Layout;
use crate::uuid::Uuid;

pub struct TypedValue {
    value_type: usize,
    value: usize,
}
pub struct KeyedTypedValue {
    pub key: String,
    pub value_type: usize,
    pub value: usize,
}
impl TypedValue {
    pub fn null() -> TypedValue {
        TypedValue {
            value_type: 0,
            value: 0,
        }
    }
    pub fn bool(value: bool) -> TypedValue {
        TypedValue {
            value_type: 1,
            value: value as usize,
        }
    }
    pub fn u8(value: u8) -> TypedValue {
        TypedValue {
            value_type: 2,
            value: value as usize,
        }
    }
    pub fn i8(value: i8) -> TypedValue {
        TypedValue {
            value_type: 3,
            value: value as usize,
        }
    }

    pub fn u16(value: u16) -> TypedValue {
        TypedValue {
            value_type: 4,
            value: value as usize,
        }
    }
    pub fn i16(value: i16) -> TypedValue {
        TypedValue {
            value_type: 5,
            value: value as usize,
        }
    }

    pub fn u32(value: u32) -> TypedValue {
        TypedValue {
            value_type: 6,
            value: value as usize,
        }
    }
    pub fn i32(value: i32) -> TypedValue {
        TypedValue {
            value_type: 7,
            value: value as usize,
        }
    }
    pub fn u64(value: u64) -> TypedValue {
        TypedValue {
            value_type: 8,
            value: value as usize,
        }
    }
    pub fn i64(value: i64) -> TypedValue {
        TypedValue {
            value_type: 9,
            value: value as usize,
        }
    }
    pub fn uuid(value: Uuid) -> TypedValue {
        TypedValue {
            value_type: 10,
            value: unsafe{Box::into_raw(Box::from(value)) as *const u8 as usize},
        }
    }
    pub fn string(value: String) -> TypedValue {
        TypedValue {
            value_type: 11,
            value: unsafe{Box::into_raw(Box::from(value)) as *const String as usize},
        }
    }
    pub fn vector(values: &[TypedValue]) -> TypedValue {
        unsafe {
            let buff = alloc::alloc::alloc(Layout{
                size:values.len() * core::mem::size_of::<TypedValue>() + core::mem::size_of::<usize>(),
                align:core::mem::align_of::<TypedValue>(),
            });
            (buff as *mut usize).write(values.len());
            let valuesPtr = ((buff as *mut usize)).offset(1) as *mut TypedValue;
            for value in values {
                valuesPtr.write_volatile(*value)
            }
            TypedValue {
                value_type: 12,
                value: buff as usize,
            }
        }
    }
    pub fn structure(values: &[KeyedTypedValue]) -> TypedValue {
        unsafe {
            let buff = alloc::alloc::alloc(Layout{
                size:values.len() * core::mem::size_of::<KeyedTypedValue>() + core::mem::size_of::<usize>(),
                align:core::mem::align_of::<KeyedTypedValue>(),
            });
            (buff as *mut usize).write(values.len());
            let valuesPtr = ((buff as *mut usize)).offset(1) as *mut KeyedTypedValue;
            for value in values {
                valuesPtr.write_volatile(*value)
            }
            TypedValue {
                value_type: 13,
                value: buff as usize,
            }
        }
    }
    pub fn to_string_verbose(&self)->String{
        if(self.value_type == 0){
            return String::from("null");
        }else if(self.value_type == 11){
            return unsafe{(*((self.value as *const String))).clone()};
        }
        else{
            return String::from("other type TODO");
        }
    }
}
impl KeyedTypedValue {
    pub fn new(key: String, value: TypedValue) -> KeyedTypedValue {
        KeyedTypedValue {
            key,
            value_type: value.value_type,
            value: value.value,
        }
    }

}