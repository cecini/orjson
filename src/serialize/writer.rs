// SPDX-License-Identifier: (Apache-2.0 OR MIT)

// use crate::ffi::{PyBytesObject, PyBytes_GET_SIZE, Py_SET_SIZE};
use crate::ffi::{PyBytesObject, PyBytes_GET_SIZE};
use core::ptr::NonNull;
use pyo3::ffi::*;
use std::os::raw::c_char;
use ::std::intrinsics::breakpoint;
use std::fmt;
use std::ffi::CString;

use std::str;


pub struct BytesWriter {
    cap: usize,
    len: usize,
    bytes: *mut PyBytesObject,
}

impl BytesWriter {
    #[inline]
    pub fn new() -> Self {
        let buf = [0; 64];
        BytesWriter {
            cap: 64,
            // pointer offset 
            len: 0,
            bytes: unsafe { PyBytes_FromStringAndSize(buf.as_ptr(), 64) as *mut PyBytesObject },
        }
    }

    #[inline]
    pub fn finish(&mut self) -> NonNull<PyObject> {
        unsafe {
            (*self.bytes.cast::<PyVarObject>()).ob_size = self.len as Py_ssize_t;
      //       // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizebeforsetobsize{}",_n);
      //      let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("setobsizeinfinishbefor{}",_n);
      //      (*self.bytes).ob_size = self.len as isize;
      //       // // why must exsit!!
      //       // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeaftersetobsize{}",_n);
      //      let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("setobsizeinfinisafter{}",_n);
      //      // Py_DECREF(self.bytes.cast::<PyObject>());

      //      // Py_REFCNT = 1
      //      // no inpact
      //      // (*self.bytes).ob_refcnt = 1 as isize;
      //      (*self.bytes.cast::<PyObject>()).ob_refcnt = 1 as isize;
      //      // set_refcnt
      //      // let  Py_REFCNT(self.bytes.cast::<PyObject>()) = 1 as iszie;
      //       // // why must exsit!!
            // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("setobsizeinfinishbefor{}",_n);


            // first it is pointer ,not add * 
            // if PyObject_HasAttrString(self.bytes.cast::<PyObject>(), "ob_size" as *const i8) == 1  {
            //    PyUnicode_InternFromString("orjson\0".as_ptr() as *const c_char),
            // if PyObject_HasAttrString(self.bytes.cast::<PyObject>(), "ob_size\0".as_ptr() as *const i8) == 1  {

            // this no size attr
            //if PyObject_HasAttrString(self.bytes.cast::<PyObject>(), "ob_size\0".as_ptr() as *const c_char) == 1  {
            //    println!("have ob_size");
            //}


           // println!("have ob_size?");
           //  println!("{}",PyObject_HasAttrString(self.bytes.cast::<PyObject>(), "ob_size\0".as_ptr() as *const c_char));

            // (*self.bytes).ob_size = self.len as Py_ssize_t;
            //
            // for py39 this is macro ,so undefined, cannot import,
            // Py_SET_SIZE(self.bytes.cast::<PyVarObject>(), self.len as Py_ssize_t);
//            (*self.bytes.cast::<PyVarObject>()).ob_size = self.len as Py_ssize_t;


            //Py_SIZE(self.bytes.cast::<PyObject>()) = self.len as Py_ssize_t;

            // the two safe 
           // println!("pysetsizeafter:{}",Py_SIZE(self.bytes.cast::<PyObject>()));
           //  println!("pysetsizeafter:{}",PyBytes_Size(self.bytes.cast::<PyObject>()));
            //(*self.bytes.cast::<PyVarObject>()).ob_size = self.len as Py_ssize_t;

             // // why must exsit!!
            // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("setobsizeinfinisafter{}",_n);
            // Py_DECREF(self.bytes.cast::<PyObject>());

            // Py_REFCNT = 1
            // no inpact
            // (*self.bytes).ob_refcnt = 1 as isize;
            // (*self.bytes.cast::<PyObject>()).ob_refcnt = 1 as isize;
            //(*self.bytes.cast::<PyObject>()).ob_refcnt = 1 as Py_ssize_t;
          //  Py_SET_REFCNT(self.bytes.cast::<PyObject>(), 1 as Py_ssize_t);
            // set_refcnt
            // let  Py_REFCNT(self.bytes.cast::<PyObject>()) = 1 as iszie;
             // // why must exsit!!

            
      //      // // Py_SET_SIZE(self.bytes.cast::<PyVarObject>(), self.len as Py_ssize_t);

      //      // comp mode ,change pyo3 
      //      // Py_SET_SIZE(self.bytes.cast::<PyVarObject>(), self.len as isize);
      //      // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeafterpysetsize{}",_n);
     
      //      // Py_SIZE(self.bytes.cast::<PyObject>()) = self.len as isize;
      //      // (*(ob as *mut PyVarObject)).ob_size

      //      let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeinfinishbefordecrefafter{}",_n);
            // Py_SIZE(self.bytes.cast::<PyObject>()) = self.len as isize;
            // (*(ob as *mut PyVarObject)).ob_size

            // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeinfinishbefordecrefafter{}",_n);
            // must set why
            self.resize(self.len as isize);
            //println!("resize after:{}",PyBytes_Size(self.bytes.cast::<PyObject>()));
            //println!("resize after:{}",Py_SIZE(self.bytes.cast::<PyObject>()));
            //println!("resize after:{}", PyBytes_GET_SIZE(self.bytes.cast::<PyObject>()));

             
            let res: NonNull<PyObject> = NonNull::new_unchecked(self.bytes as *mut PyObject);
                
            // type [i8;1]
            println!("finshed :buf bytes,.ob_sval is: {:?}", (*self.bytes.cast::<PyBytesObject>()).ob_sval);
            let buffer = std::mem::transmute::<&[c_char; 1], *mut u8>(
                // why derece the raw poiner and again ref(same as the c_char ref)
                &(*self.bytes.cast::<PyBytesObject>()).ob_sval
                //&(*self.bytes.cast::<PyBytesObject>()).ob_sval,
            );
            // println!("finshed :buf bytes,.ob_sval offset is: {:?}", (*self.bytes.cast::<PyBytesObject>()).ob_sval.offset(1) as char);
            println!("finshed :buf bytes,.ob_sval buffer as *mut c_charis: {:?}", (*buffer) as *mut c_char);
            println!("finshed :buf bytes,.ob_sval buffer *buffer is: {:#?}", (*buffer));
            println!("finshed :buf bytes,.ob_sval buffer only buffer is: {:#?}", buffer);


           //  println!("finshed :buf bytes,.ob_sval is: {:?}", (*self.bytes.cast::<PyBytesObject*>()).ob_sval);
            println!("finshed :buf bytes,.ob_sval is: {:?}", (*self.bytes.cast::<PyBytesObject>()).ob_sval);


           // The pointer refers to the internal buffer of o, which consists of len(o) + 1 bytes
            // println!("finsehd: buf bytes, AsString is: {:?}", PyBytes_AsString(self.bytes.cast::<PyObject>()));
            println!("finsehd: buf bytes, AsString is: {:?}", *PyBytes_AsString(self.bytes.cast::<PyObject>()));

            // why only one byte
            println!("finsehd: buf bytes, AsString *mut c_char is: {:#?}", *PyBytes_AsString(self.bytes.cast::<PyObject>()) as *mut c_char);
            // println!("finsehd: buf bytes, AsString is: {:#?}", PyBytes_AsString(self.bytes.cast::<PyObject>()));


            println!("finished :buf bytes,.buffer_ptr: {:?}", *self.buffer_ptr());

            // why this error 
            // Fatal Python error: _Py_CheckFunctionResult: a function returned a result with an error set
            // Python runtime state: initialized
           //SystemError: Objects/bytesobject.c:1224: bad argument to internal function

           // The above exception was the direct cause of the following exception:

            //   SystemError: <built-in function dumps> returned a result with an error set
            // let buffer: *mut *mut c_char = std::ptr::null_mut();
            // let buffer: *mut *mut c_char = CString::new(" 
            // let c_string = CString::new("hello").expect("CString::new failed");

            // let buffer: *mut *mut c_char = &mut c_string ;
            //let raw = c_string.into_raw();
            // & ref;;
            //let buffer: *mut *mut c_char = &raw;

            // // let len : *mut Py_ssize_t = 0 ;
            // let len : *mut Py_ssize_t = std::ptr::null_mut() ;
            // PyBytes_AsStringAndSize(self.bytes.cast::<PyObject>(), buffer, len); 
            // println!("finished: buf bytes，Assting and len{}{}", buffer, len);
            //if !buffer.is_null() && !len.is_null() { 
            //    println!("finished: bufbytes and size is : {:#?} size: {:?}", *buffer, *len);
            //}
           // res
            NonNull::new_unchecked(self.bytes as *mut PyObject)

        }
    }

    // u8 size count offeset 
    #[inline]
    fn buffer_ptr(&self) -> *mut u8 {
        unsafe {
            //rust array 
            //type c_char = i8
            std::mem::transmute::<&[c_char; 1], *mut u8>(
                // why derece the raw poiner and again ref(same as the c_char ref)
                &(*self.bytes.cast::<PyBytesObject>()).ob_sval,
            )
            .offset(self.len as isize)
          // primitive pointer offset .


          // let res: *mut u8 = std::mem::transmute::<&[c_char; 1], *mut u8>(
           //     &(*self.bytes.cast::<PyBytesObject>()).ob_sval,
           // )
           // .offset(self.len as isize);
            

            // let n: isize = Py_REFCNT(*mut self.bytes as mut PyBytesObject as mut PyObject);
            // let n: isize = Py_REFCNT(self.bytes as PyObject);
            // let n: isize = Py_REFCNT(* self.bytes);
            // let n: isize = Py_REFCNT(self.bytes as PyObject);
           // let n: isize = Py_REFCNT(self.bytes.0.get());
            //let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>());
            //let mut un : u8 = 2;
            //&mut un
           // res 
        }
    }

    #[inline]
    pub fn resize(&mut self, len: isize) {
        // unsafe { breakpoint() };
        unsafe {
            // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>());
            let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resize before: refcnt{}",_n);
            // int _PyBytes_Resize(PyObject **bytes, Py_ssize_t newsize)¶

            _PyBytes_Resize(
                &mut self.bytes as *mut *mut PyBytesObject as *mut *mut PyObject,
                len as isize,
            );
            println!("resize after: the size: {}",Py_SIZE(self.bytes.cast::<PyObject>()));
            // println!("reszie after :buf bytes is: {:?}", *PyBytes_AsString(self.bytes.cast::<PyObject>()));
            println!("reszie after :buf bytes,ob_sval is: {:?}", PyBytes_AsString(self.bytes.cast::<PyObject>()));

            // println!("reszie after :buf bytes,.ob_sval is: {:?}", self.bytes.ob_sval);
            // self.bytes raw ponter 
            println!("reszie after :buf bytes,.ob_sval is: {:?}", (*self.bytes).ob_sval);
            // let buffer: *mut *mut c_char = std::ptr::null;
            //let buffer: *mut *mut c_char = std::ptr::null_mut();
            // // let len : *mut Py_ssize_t = 0 ;
            // let len : *mut Py_ssize_t = std::ptr::null_mut() ;
            // PyBytes_AsStringAndSize(self.bytes.cast::<PyObject>(), buffer, len); 
            // // if !buffer.is_null() and !len.is_null() { 
            // if !buffer.is_null() && !len.is_null() { 
            //   println!("bufsize is : {:#?} size: {:?}", *buffer, *len);
            //}


            // maybe null here, have gone 
            // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeafter{}",_n);
        }
    }

    #[inline]
    pub fn prefetch(&self) {
        println!("bytes prefetch buffer_ptr"); 
        unsafe { core::intrinsics::prefetch_write_data(self.buffer_ptr(), 3) };
    }

    #[inline]
    fn grow(&mut self, len: usize) {
        while self.cap - self.len < len {
            self.cap *= 2;
        }
        unsafe {
        let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeingrowbefor{}",_n);
        };
        self.resize(self.cap as isize);
    }
}

impl std::io::Write for BytesWriter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        
        let to_write = buf.len();
        
        println!("byteswriter;s to write size for buf{}", to_write); 
        println!("byteswriter;s to write  buf assi code{:?}", buf); 
        println!("byteswriter;s to write  buf name {:?}", str::from_utf8(buf)); 
        if unlikely!(self.len + to_write > self.cap) {
            self.grow(to_write);
        }
        unsafe {
            // T type buf.as_ptr() u8, dst self.buffer_ptr, count to_write
            std::ptr::copy_nonoverlapping(buf.as_ptr() as *const u8, self.buffer_ptr(), to_write);
        };
        self.len += to_write;
        println!("byteswriter's len:{}", self.len); 
        Ok(to_write)
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error> {
        println!("byteswriter's to writeall buff  {:?}", str::from_utf8(buf)); 
        // unsafe { breakpoint() };
        self.write(buf).unwrap();
        Ok(())
    }
    #[inline]
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}
