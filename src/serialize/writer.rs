// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::PyBytesObject;
use core::ptr::NonNull;
use pyo3::ffi::*;
use std::os::raw::c_char;
// use ::std::intrinsics::breakpoint;
use std::fmt;

pub struct BytesWriter {
    cap: usize,
    len: usize,
    pub bytes: *mut PyBytesObject,
}

impl BytesWriter {
    #[inline]
    pub fn new() -> Self {
        let buf = [0; 64];
        BytesWriter {
            cap: 64,
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

            
      //      // // Py_SET_SIZE(self.bytes.cast::<PyVarObject>(), self.len as Py_ssize_t);

      //      // comp mode ,change pyo3 
      //      // Py_SET_SIZE(self.bytes.cast::<PyVarObject>(), self.len as isize);
      //      // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeafterpysetsize{}",_n);
     
      //      // Py_SIZE(self.bytes.cast::<PyObject>()) = self.len as isize;
      //      // (*(ob as *mut PyVarObject)).ob_size

      //      let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeinfinishbefordecrefafter{}",_n);
            self.resize(self.len as isize);
            NonNull::new_unchecked(self.bytes as *mut PyObject)
        }
    }

    #[inline]
    fn buffer_ptr(&self) -> *mut u8 {
        unsafe {

            std::mem::transmute::<&[c_char; 1], *mut u8>(
                &(*self.bytes.cast::<PyBytesObject>()).ob_sval,
            )
            .offset(self.len as isize)
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
            let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizebefor{}",_n);
            // int _PyBytes_Resize(PyObject **bytes, Py_ssize_t newsize)¶

            _PyBytes_Resize(
                &mut self.bytes as *mut *mut PyBytesObject as *mut *mut PyObject,
                len as isize,
            );
            // maybe null here, have gone 
            // let _n: isize = Py_REFCNT(self.bytes.cast::<PyObject>()); println!("resizeafter{}",_n);
        }
    }

    #[inline]
    pub fn prefetch(&self) {
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
        if unlikely!(self.len + to_write > self.cap) {
            self.grow(to_write);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr() as *const u8, self.buffer_ptr(), to_write);
        };
        self.len += to_write;
        Ok(to_write)
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error> {
        self.write(buf).unwrap();
        Ok(())
    }
    #[inline]
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}
