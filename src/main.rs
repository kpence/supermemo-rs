use std::io;
use std::ptr;
use std::mem;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::ffi::CString;

use winapi::um::winnt::HANDLE;
use winapi::um::errhandlingapi as werror;
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::memoryapi as wmem;
use winapi::um::processthreadsapi as wproc;
use winapi::um::psapi;
use winapi::um::handleapi as whandle;
use winapi::um::libloaderapi as wload;
use winapi::um::synchapi;
use winapi::um::winnt::{
    MEM_RESERVE,
    MEM_COMMIT,
    PAGE_EXECUTE_READWRITE,
    PROCESS_ALL_ACCESS,
};

use log::debug;
use widestring::WideCString;

macro_rules! werr {
    ($cond:expr) => {
        if $cond {
            let e = io::Error::last_os_error();
            log::error!("windows error: {:?}", e);
            return Err(e);
        }
    };
}

pub fn spawn_process(exe_path: &str) -> io::Result<HANDLE> {
    let mut proc_info = wproc::PROCESS_INFORMATION {
        hProcess: ptr::null_mut(),
        hThread: ptr::null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    };
    let mut startup_info = wproc::STARTUPINFOA {
        cb: 0,
        lpReserved: ptr::null_mut(),
        lpDesktop: ptr::null_mut(),
        lpTitle: ptr::null_mut(),
        dwX: 0,
        dwY: 0,
        dwXSize: 0,
        dwYSize: 0,
        dwXCountChars: 0,
        dwYCountChars: 0,
        dwFillAttribute: 0,
        dwFlags: 0,
        wShowWindow: 0,
        cbReserved2: 0,
        lpReserved2: ptr::null_mut(),
        hStdInput: ptr::null_mut(),
        hStdOutput: ptr::null_mut(),
        hStdError: ptr::null_mut(),
    };
    let cmd = CString::new(exe_path).unwrap();
    let result = unsafe {
        wproc::CreateProcessA(
            cmd.into_raw(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            CREATE_NO_WINDOW,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut startup_info,
            &mut proc_info,
        )
    };
    let error = unsafe { werror::GetLastError() };
    println!("Result: {}, ErrorCode: {}, processId: {}", result, error, proc_info.dwProcessId);
    werr!(proc_info.hThread.is_null());
    werr!(proc_info.hProcess.is_null());

    let handle = unsafe {
        wproc::OpenProcess(PROCESS_ALL_ACCESS, 0, proc_info.dwProcessId)
    };
    Ok(handle)
}

pub fn inject(proc: HANDLE, dll: &Path) -> io::Result<()> {
    let full_path = dll.canonicalize()?;
    let full_path = full_path.as_os_str();
    let full_path = WideCString::from_str(full_path)
        .map_err(|e| Error::new(ErrorKind::InvalidInput,
                                format!("invalid dll path: {:?}", e)))?;

    let path_len = (full_path.len() * 2) +1;
    // allocate space for the path inside target proc
    let dll_addr = unsafe {
        wmem::VirtualAllocEx(proc,
                             ptr::null_mut(),
                             path_len,
                             MEM_RESERVE | MEM_COMMIT,
                             PAGE_EXECUTE_READWRITE)
    };

    werr!(dll_addr.is_null());
    println!("allocated remote memory @ {:?}", dll_addr);

    let res = unsafe {
        // write dll inside target process
        wmem::WriteProcessMemory(proc,
                                 dll_addr,
                                 full_path.as_ptr() as *mut _,
                                 path_len,
                                 ptr::null_mut())
    };

    // TODO Testing by reading the process memory at this point?

    werr!(res == 0);

    let loadlib = unsafe {
        wload::GetProcAddress(
            wload::GetModuleHandleA("kernel32.dll\0".as_ptr() as _),
            "LoadLibraryW\0".as_ptr() as _,
        )
    };
    println!("found LoadLibraryW for injection @ {:?}", loadlib);

    let hthread = unsafe {
        wproc::CreateRemoteThread(proc, ptr::null_mut(), 0,
                                  Some(mem::transmute(loadlib)),
                                  dll_addr, 0, ptr::null_mut())
    };
    std::thread::sleep(std::time::Duration::from_millis(100));

    werr!(hthread.is_null());
    println!("spawned remote thread @ {:?}", hthread);
    unsafe {
        //synchapi::WaitForSingleObject(hthread, winapi::um::winbase::INFINITE);
        synchapi::WaitForSingleObject(hthread, 10000);

        // the next few line are for checking which dlls are being used by the target process
        let mut modules = vec![0u32; 1024];
        let mut bneeded: u32 = 0;
        psapi::EnumProcessModulesEx(proc, modules.as_ptr() as *mut _, 1024 * 4, &mut bneeded, psapi::LIST_MODULES_ALL);
        println!("bneeded: {}", bneeded);

        for i in 0..(bneeded/4) {
            let mut arr2 = vec![0u8; 1024];
            let _len = psapi::GetModuleFileNameExA(proc, modules[i as usize] as *mut _, arr2.as_mut_ptr() as *mut i8, 1024);
            let filename = String::from_utf8(arr2).unwrap().to_string();
            if filename.contains("hook.dll") {
                println!("-{}", filename.trim());
            }
        }
        wproc::ResumeThread(hthread);
        synchapi::WaitForSingleObject(hthread, 100);
        whandle::CloseHandle(hthread);
    }

    println!("Reached end of inject function");
    Ok(())
}

fn main() {
    // Print text to the console
    println!("Starting process...");
    match spawn_process("C:\\Users\\wineuser\\target.exe") {
        Ok(handle) => {
            println!("Injecting now...");
            match inject(handle, Path::new("C:\\Users\\wineuser\\hook.dll")) {
                Ok(_) => println!("Maybe successful injection"),
                Err(e) => panic!("Error in inject function: {}", e)
            }
        },
        Err(e) => {
            panic!("Opening process failed somehow. Error: {}", e)
        },
    }
    std::thread::sleep(std::time::Duration::from_secs(10));
    ()
}
