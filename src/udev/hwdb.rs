use udev::{
    libudev_c,
    iterator
};

use udev::iterator::MappedIterator;
use udev::udev::Udev;

#[doc(hidden)]
pub type HwdbIterator<'p> = MappedIterator<'p, Hwdb<'p>, (&'p str, &'p str)>;

pub struct Hwdb<'u> {
    pub udev: &'u Udev,
    hwdb: libudev_c::udev_hwdb
}

pub struct HwdbQuery<'h, 'u: 'h> {
    pub hwdb: &'h mut Hwdb<'u>,
    entry: libudev_c::udev_list_entry
}

// Crate Private
pub unsafe fn hwdb(udev: &Udev, hwdb: libudev_c::udev_hwdb) -> Hwdb {
    Hwdb { udev: udev, hwdb: hwdb }
}

impl<'u> Hwdb<'u> {
    /// Query the hardware database.
    ///
    /// # Note
    ///
    /// Only one query can exist at a time.
    pub fn query<'s>(&'s mut self, modalias: &str) -> HwdbQuery<'s, 'u> {
        // HACK: take reference here because we can't reference self.hwdb inside the closure.
        let entry = modalias.with_c_str(|modalias| {
            unsafe { libudev_c::udev_hwdb_get_properties_list_entry(self.hwdb, modalias) }
        });

        HwdbQuery {
            hwdb: self,
            entry: entry
        }
    }
}

impl<'h, 'u> HwdbQuery<'h, 'u> {
    /// Iterate over the properties returned by this query.
    pub fn iter(&self) -> HwdbIterator {
        unsafe {
            iterator::iterator(self.hwdb, self.entry)
        }.map(|(_, key, value)| (key, value.unwrap()))
    }
}

#[unsafe_destructor]
impl<'u> Drop for Hwdb<'u> {
    fn drop(&mut self) {
        unsafe { libudev_c::udev_hwdb_unref(self.hwdb) };
    }
}
