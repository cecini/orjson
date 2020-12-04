// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::*;
use crate::opt::*;
use crate::serialize::dataclass::*;
use crate::serialize::datetime::*;
use crate::serialize::default::*;
use crate::serialize::dict::*;
use crate::serialize::int::*;
use crate::serialize::list::*;
use crate::serialize::numpy::*;
use crate::serialize::str::*;
use crate::serialize::tuple::*;
use crate::serialize::uuid::*;
use crate::serialize::writer::*;
use crate::typeref::*;
use crate::unicode::*;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::io::Write;
use std::ptr::NonNull;
use pyo3::ffi::*;
use std::fmt;
use std::str;
use ::std::intrinsics::breakpoint;

pub const RECURSION_LIMIT: u8 = 255;

pub fn serialize(
    ptr: *mut pyo3::ffi::PyObject,// raw pointer ,not unsafe,from tupleGETITEM, tuple[0]
    default: Option<NonNull<pyo3::ffi::PyObject>>,
    opts: Opt,
) -> Result<NonNull<pyo3::ffi::PyObject>, String> {
    // from local serialize::writer
    let mut buf = BytesWriter::new();
    println!("in serialize, pyobject item from tuple");
    unsafe { _PyObject_Dump(ptr);}
    let obtype = pyobject_to_obtype(ptr, opts);
    // primite tyep raw pointer ,so have cast, PyBytesObject now  
    // Py_REFCNT is rust func, not c interface 
    // unsafe { let _n: isize = Py_REFCNT(buf.bytes.cast::<PyObject>()); println!("beforebufresize{}",_n);}
    match obtype {
        ObType::List | ObType::Dict | ObType::Dataclass | ObType::NumpyArray => {
            buf.resize(1024);
        }
        _ => {}
    }
    // unsafe { let _n: isize = Py_REFCNT(buf.bytes.cast::<PyObject>()); println!("afterbufresizeandbeforebufprefetch{}",_n);}
    // buf.prefetch();
    // unsafe { let _n: isize = Py_REFCNT(buf.bytes.cast::<PyObject>()); println!("afterprefetch{}",_n);}
    
    //  create type for json 
    //  size form ptr pyobject
    let obj = PyObjectSerializer::with_obtype(ptr, obtype, opts, 0, 0, default);
    // unsafe { let _n: isize = Py_REFCNT(buf.bytes.cast::<PyObject>()); println!("aftercreateobj{}",_n);}
    // should wrapper as rust fun ，then call in gdb
    unsafe { _PyObject_Dump(obj.ptr);}
    println!("in serialize, have create the PyObjectSerializer obj with  the  pyobject item ,then serde_json write to writerbuf using this obj");
    let res;
    if likely!(opts & INDENT_2 != INDENT_2) {
        // println!("serde_json's to write buff  {:?}", str::from_utf8(&obj)); 
        println!("in serialize, write to buf using the PyObjectSerializer obj by serde_json write");
        res = serde_json::to_writer(&mut buf, &obj);
    } else {
        println!("in serialize, write to buf using the PyObjectSerializer obj by serde_json pretty write");
        res = serde_json::to_writer_pretty(&mut buf, &obj);
    }
    // unsafe { let _n: isize = Py_REFCNT(buf.bytes.cast::<PyObject>()); println!("afterjsonwrite{}",_n);}
    match res {
        Ok(_) => {
            if opts & APPEND_NEWLINE != 0 {
                buf.write(b"\n").unwrap();
            }
            // unsafe { let _n: isize = Py_REFCNT(buf.bytes.cast::<PyObject>()); println!("beforebuffinish{}",_n);}
            Ok(buf.finish())
        }
        Err(err) => {
            ffi!(_Py_Dealloc(buf.finish().as_ptr()));
            Err(err.to_string())
        }
    }
}

#[derive(Copy, Clone)]
pub enum ObType {
    Str,
    Int,
    Bool,
    None,
    Float,
    List,
    Dict,
    Datetime,
    Date,
    Time,
    Tuple,
    Uuid,
    Dataclass,
    NumpyScalar,
    NumpyArray,
    Enum,
    StrSubclass,
    Unknown,
}

#[inline]
pub fn pyobject_to_obtype(obj: *mut pyo3::ffi::PyObject, opts: Opt) -> ObType {
    unsafe {
        // where this ob_type macro, src/utils 
        let ob_type = ob_type!(obj);
        println!("the obj type : {:?}", ob_type);
        if ob_type == STR_TYPE {
            ObType::Str
        } else if ob_type == INT_TYPE {
            ObType::Int
        } else if ob_type == BOOL_TYPE {
            ObType::Bool
        } else if ob_type == NONE_TYPE {
            ObType::None
        } else if ob_type == FLOAT_TYPE {
            ObType::Float
        } else if ob_type == LIST_TYPE {
            ObType::List
        } else if ob_type == DICT_TYPE {
            ObType::Dict
        } else if ob_type == DATETIME_TYPE && opts & PASSTHROUGH_DATETIME == 0 {
            ObType::Datetime
        } else {
            pyobject_to_obtype_unlikely(obj, opts)
        }
    }
}

macro_rules! is_subclass {
    ($ob_type:expr, $flag:ident) => {
        unsafe { (((*$ob_type).tp_flags & pyo3::ffi::$flag) != 0) }
    };
}

#[inline(never)]
pub fn pyobject_to_obtype_unlikely(obj: *mut pyo3::ffi::PyObject, opts: Opt) -> ObType {
    unsafe {
        let ob_type = ob_type!(obj);
        if ob_type == DATE_TYPE && opts & PASSTHROUGH_DATETIME == 0 {
            ObType::Date
        } else if ob_type == TIME_TYPE && opts & PASSTHROUGH_DATETIME == 0 {
            ObType::Time
        } else if ob_type == TUPLE_TYPE {
            ObType::Tuple
        } else if ob_type == UUID_TYPE {
            ObType::Uuid
        } else if (*(ob_type as *mut LocalPyTypeObject)).ob_type == ENUM_TYPE {
            ObType::Enum
        } else if opts & PASSTHROUGH_SUBCLASS == 0
            && is_subclass!(ob_type, Py_TPFLAGS_UNICODE_SUBCLASS)
        {
            ObType::StrSubclass
        } else if opts & PASSTHROUGH_SUBCLASS == 0
            && is_subclass!(ob_type, Py_TPFLAGS_LONG_SUBCLASS)
        {
            ObType::Int
        } else if opts & PASSTHROUGH_SUBCLASS == 0
            && is_subclass!(ob_type, Py_TPFLAGS_LIST_SUBCLASS)
        {
            ObType::List
        } else if opts & PASSTHROUGH_SUBCLASS == 0
            && is_subclass!(ob_type, Py_TPFLAGS_DICT_SUBCLASS)
        {
            ObType::Dict
        } else if opts & PASSTHROUGH_DATACLASS == 0
            && ffi!(PyDict_Contains((*ob_type).tp_dict, DATACLASS_FIELDS_STR)) == 1
        {
            ObType::Dataclass
        } else if opts & SERIALIZE_NUMPY != 0 && is_numpy_scalar(ob_type) {
            ObType::NumpyScalar
        } else if opts & SERIALIZE_NUMPY != 0 && is_numpy_array(ob_type) {
            ObType::NumpyArray
        } else {
            ObType::Unknown
        }
    }
}

pub struct PyObjectSerializer {
    ptr: *mut pyo3::ffi::PyObject,
    obtype: ObType,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl PyObjectSerializer {
    #[inline]
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        PyObjectSerializer {
            ptr: ptr,
            obtype: pyobject_to_obtype(ptr, opts),
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }

    #[inline]
    pub fn with_obtype(
        ptr: *mut pyo3::ffi::PyObject,
        obtype: ObType,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        PyObjectSerializer {
            ptr: ptr,
            obtype: obtype,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl<'p> Serialize for PyObjectSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        println!("obj PyObjectSerializer::Serialize() self.ptr dump!!!{:?}", ob_type!(self.ptr)); 
        unsafe { _PyObject_Dump(self.ptr);}
        match self.obtype {
            ObType::Str => {
                let mut str_size: pyo3::ffi::Py_ssize_t = 0;
                let uni = read_utf8_from_str(self.ptr, &mut str_size);
                if unlikely!(uni.is_null()) {
                    err!(INVALID_STR)
                }
                serializer.serialize_str(str_from_slice!(uni, str_size))
            }
            ObType::StrSubclass => StrSubclassSerializer::new(self.ptr).serialize(serializer),
            ObType::Int => IntSerializer::new(self.ptr, self.opts).serialize(serializer),
            ObType::None => serializer.serialize_unit(),
            ObType::Float => serializer.serialize_f64(ffi!(PyFloat_AS_DOUBLE(self.ptr))),
            ObType::Bool => serializer.serialize_bool(unsafe { self.ptr == TRUE }),
            ObType::Datetime => DateTime::new(self.ptr, self.opts).serialize(serializer),
            ObType::Date => Date::new(self.ptr).serialize(serializer),
            ObType::Time => match Time::new(self.ptr, self.opts) {
                Ok(val) => val.serialize(serializer),
                Err(TimeError::HasTimezone) => err!(TIME_HAS_TZINFO),
            },
            ObType::Uuid => UUID::new(self.ptr).serialize(serializer),
            ObType::Dict => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                // unsafe { breakpoint() };
                let len = unsafe { 
                    let a =  self.ptr.cast::<pyo3::ffi::PyDictObject>();
                    PyDict_GET_SIZE(self.ptr) as usize 
                };
                println!("dict len: {}", len);
                if unlikely!(len == 0) {
                    serializer.serialize_map(Some(0)).unwrap().end()
                } else if likely!(self.opts & SORT_OR_NON_STR_KEYS == 0) {
                    println!("dict norm new: sort_or_non_str_key and opts == 0");
                    Dict::new(
                        self.ptr,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                        len,
                    )
                    .serialize(serializer)
                } else if self.opts & NON_STR_KEYS != 0 {
                    println!("dict non_str_key new");
                    DictNonStrKey::new(
                        self.ptr,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                        len,
                    )
                    .serialize(serializer)
                } else {
                    println!("dict sortedkey new");
                    DictSortedKey::new(
                        self.ptr,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                        len,
                    )
                    .serialize(serializer)
                }
            }
            ObType::List => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                let len = ffi!(PyList_GET_SIZE(self.ptr)) as usize;
                if unlikely!(len == 0) {
                    serializer.serialize_seq(Some(0)).unwrap().end()
                } else {
                    ListSerializer::new(
                        self.ptr,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                        len,
                    )
                    .serialize(serializer)
                }
            }
            ObType::Tuple => TupleSerializer::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer),
            ObType::Dataclass => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                let dict = ffi!(PyObject_GetAttr(self.ptr, DICT_STR));
                if likely!(!dict.is_null()) {
                    ffi!(Py_DECREF(dict));
                    DataclassFastSerializer::new(
                        dict,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                    )
                    .serialize(serializer)
                } else {
                    unsafe { pyo3::ffi::PyErr_Clear() };
                    DataclassFallbackSerializer::new(
                        self.ptr,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                    )
                    .serialize(serializer)
                }
            }
            ObType::Enum => {
                let value = ffi!(PyObject_GetAttr(self.ptr, VALUE_STR));
                ffi!(Py_DECREF(value));
                PyObjectSerializer::new(
                    value,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer)
            }
            ObType::NumpyArray => match NumpyArray::new(self.ptr) {
                Ok(val) => val.serialize(serializer),
                Err(PyArrayError::Malformed) => err!("numpy array is malformed"),
                Err(PyArrayError::NotContiguous) | Err(PyArrayError::UnsupportedDataType) => {
                    DefaultSerializer::new(
                        self.ptr,
                        self.opts,
                        self.default_calls,
                        self.recursion,
                        self.default,
                    )
                    .serialize(serializer)
                }
            },
            ObType::NumpyScalar => NumpyScalar::new(self.ptr).serialize(serializer),
            ObType::Unknown => DefaultSerializer::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer),
        }
    }
}
