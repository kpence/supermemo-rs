use std::{alloc::Layout, mem, ptr, slice};

// source: https://gist.github.com/thomcc/af7f05e308b95a1c2b935cbd1a8cf3b6
#[repr(C)]
struct UnicodeStringData {
    pub buffer_ptr: *mut u16,
    pub code_page: u16,
    pub bytes_per_char: u16,
    pub rc: u32,
    pub len: u32,
    pub buffer: [u16; 0],
    // _pin: core::marker::PhantomPinned,
}

#[repr(transparent)]
pub struct UnicodeString(*mut UnicodeStringData);

impl UnicodeString {
    /// Includes the nul terminator
    pub fn full_len(&self) -> usize {
        unsafe { ptr::addr_of!((*self.0).len).read() as usize }
    }
    pub fn len(&self) -> usize {
        let len = self.full_len();
        if len > 0 {
            len - 1
        } else {
            0
        }
    }
    pub fn as_utf16(&self) -> &[u16] {
        let full = self.as_utf16_with_nul();
        &full[..full.len() - 1]
    }
    pub fn as_utf16_with_nul(&self) -> &[u16] {
        unsafe {
            let len = self.full_len();
            assert_eq!(ptr::addr_of!((*self.0).bytes_per_char).read(), 2);
            let u16s_ptr = ptr::addr_of!((*self.0).buffer_ptr).read();
            slice::from_raw_parts(u16s_ptr, len)
        }
    }
    pub fn as_ptr(&self) -> *const () {
        self.0 as *const ()
    }
    pub fn buffer_ptr(&self) -> *mut u16 {
        unsafe { (*self.0).buffer_ptr }
    }
}

impl Default for UnicodeString {
    fn default() -> Self {
        unsafe {
            use ptr::addr_of_mut;

            let layout_arr = Layout::array::<u16>(1).unwrap();
            let (layout_string, buffer_start_offset) = Layout::new::<UnicodeStringData>()
                .extend(layout_arr)
                .unwrap();
            assert_eq!(buffer_start_offset, mem::size_of::<UnicodeStringData>());

            let s = std::alloc::alloc(layout_string) as *mut UnicodeStringData;
            if s.is_null() {
                std::alloc::handle_alloc_error(layout_string);
            }
            addr_of_mut!((*s).code_page).write(0x04b0);
            addr_of_mut!((*s).bytes_per_char).write(2);
            addr_of_mut!((*s).rc).write(1);
            addr_of_mut!((*s).len).write(0);
            addr_of_mut!((*s).buffer_ptr).write(ptr::null_mut());
            UnicodeString(s)
        }
    }
}

impl Drop for UnicodeString {
    fn drop(&mut self) {
        unsafe {
            let len = ptr::addr_of!((*self.0).len).read() as usize;
            let layout_arr = Layout::array::<u16>(len).unwrap();
            let (layout_string, _) = Layout::new::<UnicodeStringData>()
                .extend(layout_arr)
                .unwrap();
            std::alloc::dealloc(self.0 as *mut u8, layout_string);
        }
    }
}

impl From<&str> for UnicodeString {
    fn from(s: &str) -> Self {
        unsafe {
            use ptr::addr_of_mut;
            let mut utf16: Vec<u16> = s.encode_utf16().collect();
            utf16.push(0);

            let layout_arr = Layout::array::<u16>(utf16.len()).unwrap();
            let (layout_string, buffer_start_offset) = Layout::new::<UnicodeStringData>()
                .extend(layout_arr)
                .unwrap();
            assert_eq!(buffer_start_offset, mem::size_of::<UnicodeStringData>());

            let s = std::alloc::alloc(layout_string) as *mut UnicodeStringData;
            if s.is_null() {
                std::alloc::handle_alloc_error(layout_string);
            }
            addr_of_mut!((*s).code_page).write(0x04b0);
            addr_of_mut!((*s).bytes_per_char).write(2);
            addr_of_mut!((*s).rc).write(1);
            addr_of_mut!((*s).len).write(utf16.len() as u32);
            let bufptr = addr_of_mut!((*s).buffer).cast::<u16>();
            assert_eq!(
                bufptr.cast::<u8>(),
                s.cast::<u8>().wrapping_add(buffer_start_offset),
            );

            addr_of_mut!((*s).buffer_ptr).write(bufptr);
            ptr::copy_nonoverlapping(utf16.as_ptr(), bufptr, utf16.len());
            UnicodeString(s)
        }
    }
}

impl From<&UnicodeString> for String {
    fn from(u: &UnicodeString) -> Self {
        if u.full_len() == 0 {
            String::default()
        } else {
            String::from_utf16_lossy(u.as_utf16())
        }
    }
}
