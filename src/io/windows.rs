use std::ffi::OsStr;
use std::fmt;
use std::iter::once;
use std::mem::{size_of, MaybeUninit};
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;

use anyhow::Error;
use winapi::shared::minwindef::{DWORD, FALSE, TRUE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateDirectoryW, CreateFileW, CREATE_NEW};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::heapapi::{GetProcessHeap, HeapAlloc, HeapFree};
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::{
    AddAccessAllowedAce, GetLengthSid, GetTokenInformation, InitializeAcl,
    InitializeSecurityDescriptor, SetSecurityDescriptorDacl,
};
use winapi::um::winnt::{
    TokenUser, ACCESS_ALLOWED_ACE, ACL, ACL_REVISION, FILE_ATTRIBUTE_NORMAL, GENERIC_ALL,
    GENERIC_READ, GENERIC_WRITE, HANDLE, SECURITY_DESCRIPTOR, SECURITY_DESCRIPTOR_REVISION,
    TOKEN_QUERY, TOKEN_USER,
};

#[derive(Debug, thiserror::Error)]
struct WinError {
    function: String,
    code: DWORD,
}

impl fmt::Display for WinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}() error, code: {}", self.function, self.code)
    }
}

macro_rules! wintry {
    ( $i:ident ( $($x:expr),* ) ) => {
        let result = $i($($x),*);
        if result == 0 {
            return Err(WinError { function: stringify!($i).to_string(), code: GetLastError() }.into());
        }
    };
}

unsafe fn with_security_attributes<F, R>(proc: F) -> Result<R, Error>
where
    F: FnOnce(SECURITY_ATTRIBUTES) -> Result<R, Error>,
{
    let mut token_handle: HANDLE = null_mut();
    wintry!(OpenProcessToken(
        GetCurrentProcess(),
        TOKEN_QUERY,
        &mut token_handle
    ));
    let mut returned: DWORD = 0;
    GetTokenInformation(token_handle, TokenUser, null_mut(), 0, &mut returned);

    let user = HeapAlloc(GetProcessHeap(), 0, returned as usize);
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
        sd.as_mut_ptr().cast(),
        SECURITY_DESCRIPTOR_REVISION
    ));

    let acl_size = size_of::<ACL>() + size_of::<ACCESS_ALLOWED_ACE>() + GetLengthSid(sid) as usize;
    let dacl = HeapAlloc(GetProcessHeap(), 0, acl_size);
    if dacl.is_null() {
        return Err(WinError {
            function: "HeapAlloc".to_string(),
            code: GetLastError(),
        }
        .into());
    }
    wintry!(InitializeAcl(
        dacl.cast(),
        acl_size as DWORD,
        ACL_REVISION as DWORD
    ));

    wintry!(AddAccessAllowedAce(
        dacl.cast(),
        ACL_REVISION as DWORD,
        GENERIC_ALL,
        sid
    ));

    SetSecurityDescriptorDacl(sd.as_mut_ptr().cast(), TRUE, dacl.cast(), FALSE);

    let sa = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as DWORD,
        bInheritHandle: 0,
        lpSecurityDescriptor: sd.as_mut_ptr().cast(),
    };
    let _sd = sd.assume_init();

    let r = proc(sa);

    HeapFree(GetProcessHeap(), 0, dacl);
    HeapFree(GetProcessHeap(), 0, user);

    r
}

pub fn create_file_handle(path: &Path) -> Result<HANDLE, Error> {
    unsafe {
        with_security_attributes(|mut sa| {
            let handle = CreateFileW(
                osstr_to_vecu16(path.as_os_str()).as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                &mut sa,
                CREATE_NEW,
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            );
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
        with_security_attributes(|mut sa| {
            let result = CreateDirectoryW(osstr_to_vecu16(path.as_os_str()).as_ptr(), &mut sa);
            if result == TRUE {
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
