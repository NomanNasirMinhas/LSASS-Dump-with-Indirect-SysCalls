#![allow(non_snake_case)]
extern crate winapi;

use winapi::um::winnt::*;
use winapi::um::winbase::*;
use winapi::um::handleapi::*;
use winapi::um::fileapi::*;

fn main(){
    let driver_path = "\\\\.\\PROCEXP152";
    let handle = unsafe {
        CreateFileW(
            driver_path.encode_utf16().collect::<Vec<_>>().as_ptr(),
            GENERIC_ALL,
            0,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        // Handle the error here.
        // Be sure to check GetLastError() for more information.
        let err = unsafe { winapi::um::errhandlingapi::GetLastError() };
        println!("Failed to open the driver handle. {:?}", err);
    } else {
        // Successfully opened the driver. You can now send IOCTL commands or perform other actions.
        // Don't forget to close the handle when you're done with it.
       println!("Successfully opened the driver handle.");
    }

    unsafe {
        winapi::um::handleapi::CloseHandle(handle);
    }

    //wait for input
    println!("Press any key to exit.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}