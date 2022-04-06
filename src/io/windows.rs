use std::ffi::OsStr;
use std::fmt;
use std::iter::once;
use std::mem::{size_of, MaybeUninit};
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;

use anyhow::Error;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{
    CloseHandle, GetLastError, BOOL, HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR,
};
use windows::Win32::Security::{
    AddAccessAllowedAce, GetLengthSid, GetTokenInformation, InitializeAcl,
    InitializeSecurityDescriptor, SetSecurityDescriptorDacl, TokenUser, ACCESS_ALLOWED_ACE, ACL,
    ACL_REVISION, PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR, TOKEN_QUERY,
    TOKEN_USER,
};
use windows::Win32::Storage::FileSystem::{
    CreateDirectoryW, CreateFileW, CREATE_NEW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ,
    FILE_GENERIC_WRITE, FILE_SHARE_MODE,
};
use windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree, HEAP_FLAGS};
use windows::Win32::System::SystemServices::{GENERIC_ALL, SECURITY_DESCRIPTOR_REVISION};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

#[derive(Debug, thiserror::Error)]
struct WinError {
    function: String,
    code: WIN32_ERROR,
}

impl fmt::Display for WinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}() error, code: {}", self.function, self.code.0)
    }
}

macro_rules! wintry {
    ( $i:ident ( $($x:expr),* ) ) => {
        let result = $i($($x),*);
        if !result.as_bool() {
            return Err(WinError { function: stringify!($i).to_string(), code: GetLastError() }.into());
        }
    };
}

unsafe fn with_security_attributes<F, R>(proc: F) -> Result<R, Error>
where
    F: FnOnce(SECURITY_ATTRIBUTES) -> Result<R, Error>,
{
    let mut token_handle: HANDLE = HANDLE::default();
    wintry!(OpenProcessToken(
        GetCurrentProcess(),
        TOKEN_QUERY,
        &mut token_handle
    ));
    let mut returned: u32 = 0;
    GetTokenInformation(token_handle, TokenUser, null_mut(), 0, &mut returned);

    let user = HeapAlloc(GetProcessHeap()?, HEAP_FLAGS(0), returned as usize);
    if user.is_null() {
        return Err(WinError {
            function: "HeapAlloc".to_string(),
            code: GetLastError(),
        }
        .into());
    }
    wintry!(GetTokenInformation(
        token_handle,
        TokenUser,
        user.cast(),
        returned,
        &mut returned
    ));
    wintry!(CloseHandle(token_handle));

    let sid = (*user.cast::<TOKEN_USER>()).User.Sid;

    let mut sd: MaybeUninit<SECURITY_DESCRIPTOR> = MaybeUninit::zeroed();
    wintry!(InitializeSecurityDescriptor(
        PSECURITY_DESCRIPTOR(sd.as_mut_ptr().cast()),
        SECURITY_DESCRIPTOR_REVISION
    ));

    let acl_size = size_of::<ACL>() + size_of::<ACCESS_ALLOWED_ACE>() + GetLengthSid(sid) as usize;
    let dacl = HeapAlloc(GetProcessHeap()?, HEAP_FLAGS(0), acl_size);
    if dacl.is_null() {
        return Err(WinError {
            function: "HeapAlloc".to_string(),
            code: GetLastError(),
        }
        .into());
    }
    wintry!(InitializeAcl(dacl.cast(), acl_size as u32, ACL_REVISION.0));

    wintry!(AddAccessAllowedAce(
        dacl.cast(),
        ACL_REVISION.0,
        GENERIC_ALL,
        sid
    ));

    SetSecurityDescriptorDacl(
        PSECURITY_DESCRIPTOR(sd.as_mut_ptr().cast()),
        BOOL::from(true),
        dacl.cast(),
        BOOL::from(false),
    );

    let sa = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        bInheritHandle: BOOL::from(false),
        lpSecurityDescriptor: sd.as_mut_ptr().cast(),
    };
    let _sd = sd.assume_init();

    let r = proc(sa);

    HeapFree(GetProcessHeap()?, HEAP_FLAGS(0), dacl);
    HeapFree(GetProcessHeap()?, HEAP_FLAGS(0), user);

    r
}

pub fn create_file_handle(path: &Path) -> Result<HANDLE, Error> {
    unsafe {
        with_security_attributes(|sa| {
            let handle = CreateFileW(
                PCWSTR(osstr_to_vecu16(path.as_os_str()).as_mut_ptr()),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_MODE(0),
                &sa,
                CREATE_NEW,
                FILE_ATTRIBUTE_NORMAL,
                HANDLE::default(),
            )?;
            if handle == INVALID_HANDLE_VALUE {
                return Err(WinError {
                    function: "CreateFileW".to_string(),
                    code: GetLastError(),
                }
                .into());
            }
            Ok(handle)
        })
    }
}

pub fn create_directory(path: &Path) -> Result<(), Error> {
    unsafe {
        with_security_attributes(|sa| {
            let result =
                CreateDirectoryW(PCWSTR(osstr_to_vecu16(path.as_os_str()).as_mut_ptr()), &sa);
            if result.as_bool() {
                Ok(())
            } else {
                Err(WinError {
                    function: "CreateDirectoryW".to_string(),
                    code: GetLastError(),
                }
                .into())
            }
        })
    }
}

fn osstr_to_vecu16(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(once(0)).collect::<Vec<u16>>()
}
