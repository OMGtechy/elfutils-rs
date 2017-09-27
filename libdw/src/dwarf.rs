use ffi;
use libelf;
use std::ptr;

use std::fmt;
use std::marker::PhantomData;
use std::os::unix::io::AsRawFd;

use super::Result;
use super::{CompileUnits, TypeUnits};


pub struct Dwarf<'a> {
    inner: *mut ffi::Dwarf,
    owned: bool,
    phantom: PhantomData<&'a mut ffi::Dwarf>,
}

impl<'a> fmt::Debug for Dwarf<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Dwarf")
            .field("inner", &self.inner)
            .field("owned", &self.owned)
            .finish()
    }
}

impl<'a> Dwarf<'a> {
    #[inline]
    fn new(dwarf: *mut ffi::Dwarf) -> Self {
        Dwarf {
            inner: dwarf,
            owned: true,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_fd<FD: AsRawFd>(fd: &'a FD) -> Result<Dwarf<'a>> {
        let fd = fd.as_raw_fd();
        let dwarf = ffi!(dwarf_begin(fd, ffi::Dwarf_Cmd::DWARF_C_READ))?;
        Ok(Dwarf::new(dwarf))
    }

    #[inline]
    pub fn from_elf(elf: &'a libelf::Elf) -> Result<Dwarf<'a>> {
        let elf = elf.as_ptr() as *mut _; // FIXME distinct bindgen Elf types
        let dwarf = ffi!(dwarf_begin_elf(elf, ffi::Dwarf_Cmd::DWARF_C_READ, ptr::null_mut()))?;
        Ok(Dwarf::new(dwarf))
    }

    #[inline]
    pub unsafe fn from_raw(dwarf: *mut ffi::Dwarf) -> Self {
        Dwarf {
            inner: dwarf,
            owned: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn get_elf(&'a self) -> Result<libelf::Elf<'a>> {
        let elf = ffi!(dwarf_getelf(self.as_ptr()))?;
        let elf = elf as *mut _; // FIXME distinct bindgen Elf types
        Ok(unsafe { libelf::Elf::from_raw(elf) })
    }

    #[inline]
    pub fn compile_units(&'a self) -> CompileUnits<'a> {
        ::units::compile_units(self)
    }

    #[inline]
    pub fn type_units(&'a self) -> TypeUnits<'a> {
        ::units::type_units(self)
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut ffi::Dwarf {
        self.inner
    }
}

impl<'a> Drop for Dwarf<'a> {
    #[inline]
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::dwarf_end(self.inner);
            }
        }
    }
}
