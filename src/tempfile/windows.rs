use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::iter::once;
use std::mem::{size_of, MaybeUninit};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::FromRawHandle;
use std::path::{Path, PathBuf};
use std::ptr::null_mut;

use anyhow::Error;
use winapi::shared::minwindef::{DWORD, FALSE, TRUE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileW, CREATE_NEW};
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

use crate::rand::gen_random_alphanum;
use crate::tempfile::common::TempPath;

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

pub fn create_temp_file(dir: &Path) -> Result<(TempPath, File), Error> {
    unsafe {
        let temp_path = create_temp_path(dir)?;
        let handle = create_temp_handle(&temp_path)?;
        Ok((
            TempPath::new(temp_path),
            File::from_raw_handle(handle.cast()),
        ))
    }
}

unsafe fn create_temp_handle(temp_path: &Path) -> Result<HANDLE, Error> {
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
        }.into());
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
        }.into());
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

    let mut sa = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as DWORD,
        bInheritHandle: 0,
        lpSecurityDescriptor: sd.as_mut_ptr().cast(),
    };
    let _sd = sd.assume_init();
    let handle = CreateFileW(
        osstr_to_vecu16(temp_path.as_os_str()).as_ptr(),
        GENERIC_READ | GENERIC_WRITE,
        0,
        &mut sa,
        CREATE_NEW,
        FILE_ATTRIBUTE_NORMAL,
        null_mut(),
    );

    HeapFree(GetProcessHeap(), 0, dacl);
    HeapFree(GetProcessHeap(), 0, user);

    if handle == INVALID_HANDLE_VALUE {
        return Err(WinError {
            function: "CreateFileW".to_string(),
            code: GetLastError(),
        }.into());
    }
    Ok(handle)
}

fn create_temp_path(dir: &Path) -> Result<PathBuf, Error> {
    let mut pb = dir.to_path_buf();
    let mut name = String::from("aikot-");
    name.push_str(&gen_random_alphanum(6));
    pb.push(name);
    Ok(pb)
}

fn osstr_to_vecu16(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(once(0)).collect::<Vec<u16>>()
}
