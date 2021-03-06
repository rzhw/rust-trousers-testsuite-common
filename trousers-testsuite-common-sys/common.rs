extern crate libc;
extern crate trousers_sys;
use self::trousers_sys::tspi::*;

/* automatically generated by rust-bindgen */

#[link(name = "trousers-testsuite-common", kind = "static")]
extern "C" {
    pub static mut SRK_UUID: TSS_UUID;
}
#[link(name = "trousers-testsuite-common", kind = "static")]
extern "C" {
    pub fn err_string(arg1: TSS_RESULT) -> *mut libc::c_char;
    pub fn parseArgs(arg1: libc::c_int, arg2: *mut *mut libc::c_char)
     -> libc::c_char;
    pub fn print_wrongVersion() -> ();
    pub fn print_NA() -> ();
    pub fn checkNonAPI(arg1: TSS_RESULT) -> libc::c_int;
    pub fn print_wrongChar() -> ();
    pub fn UINT32ToArray(i: UINT32, out: *mut BYTE) -> ();
    pub fn get_server(arg1: *mut libc::c_char) -> *mut UNICODE;
    pub fn print_hex(arg1: *mut BYTE, arg2: UINT32) -> ();
    pub fn create_key(arg1: TSS_HCONTEXT, arg2: TSS_FLAG, arg3: TSS_HKEY,
                      arg4: *mut TSS_HKEY) -> TSS_RESULT;
    pub fn create_load_key(arg1: TSS_HCONTEXT, arg2: TSS_FLAG, arg3: TSS_HKEY,
                           arg4: *mut TSS_HKEY) -> TSS_RESULT;
    pub fn set_secret(arg1: TSS_HCONTEXT, arg2: TSS_HOBJECT,
                      arg3: *mut TSS_HPOLICY) -> TSS_RESULT;
    pub fn connect_load_srk(arg1: *mut TSS_HCONTEXT, arg2: *mut TSS_HKEY)
     -> TSS_RESULT;
    pub fn connect_load_all(arg1: *mut TSS_HCONTEXT, arg2: *mut TSS_HKEY,
                            arg3: *mut TSS_HTPM) -> TSS_RESULT;
    pub fn bind_and_unbind(arg1: TSS_HCONTEXT, arg2: TSS_HKEY) -> TSS_RESULT;
    pub fn sign_and_verify(arg1: TSS_HCONTEXT, arg2: TSS_HKEY) -> TSS_RESULT;
    pub fn seal_and_unseal(arg1: TSS_HCONTEXT, arg2: TSS_HKEY,
                           arg3: TSS_HENCDATA, arg4: TSS_HPCRS) -> TSS_RESULT;
    pub fn set_public_modulus(arg1: TSS_HCONTEXT, arg2: TSS_HKEY,
                              arg3: UINT32, arg4: *mut BYTE) -> TSS_RESULT;
    pub fn set_srk_readable(arg1: TSS_HCONTEXT) -> TSS_RESULT;
    pub fn TestSuite_LoadBlob_PUBKEY(arg1: *mut UINT16, arg2: *mut BYTE,
                                     arg3: *mut TCPA_PUBKEY) -> ();
    pub fn TestSuite_LoadBlob(arg1: *mut UINT16, arg2: UINT32,
                              arg3: *mut BYTE, arg4: *mut BYTE) -> ();
    pub fn TestSuite_UnloadBlob(arg1: *mut UINT16, arg2: UINT32,
                                arg3: *mut BYTE, arg4: *mut BYTE) -> ();
    pub fn TestSuite_LoadBlob_BYTE(arg1: *mut UINT16, arg2: BYTE,
                                   arg3: *mut BYTE) -> ();
    pub fn TestSuite_UnloadBlob_BYTE(arg1: *mut UINT16, arg2: *mut BYTE,
                                     arg3: *mut BYTE) -> ();
    pub fn TestSuite_LoadBlob_BOOL(arg1: *mut UINT16, arg2: TSS_BOOL,
                                   arg3: *mut BYTE) -> ();
    pub fn TestSuite_UnloadBlob_BOOL(arg1: *mut UINT16, arg2: *mut TSS_BOOL,
                                     arg3: *mut BYTE) -> ();
    pub fn TestSuite_LoadBlob_UINT32(arg1: *mut UINT16, arg2: UINT32,
                                     arg3: *mut BYTE) -> ();
    pub fn TestSuite_LoadBlob_UINT16(arg1: *mut UINT16, arg2: UINT16,
                                     arg3: *mut BYTE) -> ();
    pub fn TestSuite_UnloadBlob_UINT32(arg1: *mut UINT16, arg2: *mut UINT32,
                                       arg3: *mut BYTE) -> ();
    pub fn TestSuite_UnloadBlob_UINT16(arg1: *mut UINT16, arg2: *mut UINT16,
                                       arg3: *mut BYTE) -> ();
    pub fn TestSuite_LoadBlob_RSA_KEY_PARMS(arg1: *mut UINT16,
                                            arg2: *mut BYTE,
                                            arg3: *mut TCPA_RSA_KEY_PARMS)
     -> ();
    pub fn TestSuite_LoadBlob_TSS_VERSION(arg1: *mut UINT16, arg2: *mut BYTE,
                                          arg3: TSS_VERSION) -> ();
    pub fn TestSuite_UnloadBlob_TCPA_VERSION(arg1: *mut UINT16,
                                             arg2: *mut BYTE,
                                             arg3: *mut TCPA_VERSION) -> ();
    pub fn TestSuite_LoadBlob_TCPA_VERSION(arg1: *mut UINT16, arg2: *mut BYTE,
                                           arg3: TCPA_VERSION) -> ();
    pub fn TestSuite_LoadBlob_KEY(arg1: *mut UINT16, arg2: *mut BYTE,
                                  arg3: *mut TCPA_KEY) -> ();
    pub fn TestSuite_LoadBlob_KEY_PARMS(arg1: *mut UINT16, arg2: *mut BYTE,
                                        arg3: *mut TCPA_KEY_PARMS) -> ();
    pub fn TestSuite_LoadBlob_STORE_PUBKEY(arg1: *mut UINT16, arg2: *mut BYTE,
                                           arg3: *mut TCPA_STORE_PUBKEY)
     -> ();
    pub fn TestSuite_LoadBlob_SYMMETRIC_KEY(arg1: *mut UINT16,
                                            arg2: *mut BYTE,
                                            arg3: *mut TCPA_SYMMETRIC_KEY)
     -> ();
    pub fn TestSuite_UnloadBlob_SYMMETRIC_KEY(arg1: *mut UINT16,
                                              arg2: *mut BYTE,
                                              arg3: *mut TCPA_SYMMETRIC_KEY)
     -> TSS_RESULT;
    pub fn TestSuite_UnloadBlob_KEY_PARMS(arg1: *mut UINT16, arg2: *mut BYTE,
                                          arg3: *mut TCPA_KEY_PARMS)
     -> TSS_RESULT;
    pub fn TestSuite_UnloadBlob_KEY(arg1: *mut UINT16, arg2: *mut BYTE,
                                    arg3: *mut TCPA_KEY) -> TSS_RESULT;
    pub fn TestSuite_UnloadBlob_KEY12(arg1: *mut UINT16, arg2: *mut BYTE,
                                      arg3: *mut TPM_KEY12) -> TSS_RESULT;
    pub fn TestSuite_UnloadBlob_STORE_PUBKEY(arg1: *mut UINT16,
                                             arg2: *mut BYTE,
                                             arg3: *mut TCPA_STORE_PUBKEY)
     -> TSS_RESULT;
    pub fn TestSuite_UnloadBlob_PUBKEY(arg1: *mut UINT16, arg2: *mut BYTE,
                                       arg3: *mut TCPA_PUBKEY) -> TSS_RESULT;
    pub fn TestSuite_UnloadBlob_VERSION(arg1: *mut UINT16, arg2: *mut BYTE,
                                        arg3: *mut TCPA_VERSION) -> ();
    pub fn TestSuite_UnloadBlob_IDENTITY_PROOF(arg1: *mut UINT16,
                                               arg2: *mut BYTE,
                                               arg3: *mut TCPA_IDENTITY_PROOF)
     -> TSS_RESULT;
    pub fn TestSuite_LoadBlob_SYM_CA_ATTESTATION(arg1: *mut UINT16,
                                                 arg2: *mut BYTE,
                                                 arg3:
                                                     *mut TCPA_SYM_CA_ATTESTATION)
     -> ();
    pub fn TestSuite_UnloadBlob_SYM_CA_ATTESTATION(arg1: *mut UINT16,
                                                   arg2: *mut BYTE,
                                                   arg3:
                                                       *mut TCPA_SYM_CA_ATTESTATION)
     -> TSS_RESULT;
    pub fn TestSuite_LoadBlob_ASYM_CA_CONTENTS(arg1: *mut UINT16,
                                               arg2: *mut BYTE,
                                               arg3:
                                                   *mut TCPA_ASYM_CA_CONTENTS)
     -> ();
    pub fn TestSuite_UnloadBlob_ASYM_CA_CONTENTS(arg1: *mut UINT16,
                                                 arg2: *mut BYTE,
                                                 arg3:
                                                     *mut TCPA_ASYM_CA_CONTENTS)
     -> TSS_RESULT;
    pub fn TestSuite_LoadBlob_KEY_FLAGS(arg1: *mut UINT16, arg2: *mut BYTE,
                                        arg3: *mut TCPA_KEY_FLAGS) -> ();
    pub fn TestSuite_UnloadBlob_KEY_FLAGS(arg1: *mut UINT16, arg2: *mut BYTE,
                                          arg3: *mut TCPA_KEY_FLAGS) -> ();
    pub fn TestSuite_UnloadBlob_IDENTITY_REQ(arg1: *mut UINT16,
                                             arg2: *mut BYTE,
                                             arg3: *mut TCPA_IDENTITY_REQ)
     -> TSS_RESULT;
    pub fn TestSuite_Native_To_UNICODE(arg1: *mut BYTE,
                                       arg2: *mut libc::c_uint)
     -> *mut BYTE;
    pub fn TestSuite_UNICODE_To_Native(arg1: *mut BYTE,
                                       arg2: *mut libc::c_uint)
     -> *mut BYTE;
    pub fn TestSuite_SymEncrypt(alg: UINT16, mode: BYTE, key: *mut BYTE,
                                iv: *mut BYTE, _in: *mut BYTE, in_len: UINT32,
                                out: *mut BYTE, out_len: *mut UINT32)
     -> TSS_RESULT;
    pub fn TestSuite_SymDecrypt(alg: UINT16, mode: BYTE, key: *mut BYTE,
                                iv: *mut BYTE, _in: *mut BYTE, in_len: UINT32,
                                out: *mut BYTE, out_len: *mut UINT32)
     -> TSS_RESULT;
    pub fn TestSuite_RSA_Public_Encrypt(_in: *mut libc::c_uchar,
                                        inlen: libc::c_uint,
                                        out: *mut libc::c_uchar,
                                        outlen: *mut libc::c_uint,
                                        pubkey: *mut libc::c_uchar,
                                        pubsize: libc::c_uint,
                                        e: libc::c_uint,
                                        padding: libc::c_int)
     -> libc::c_int;
    pub fn TestSuite_TPM_RSA_Encrypt(_in: *mut libc::c_uchar,
                                     inlen: libc::c_uint,
                                     out: *mut libc::c_uchar,
                                     outlen: *mut libc::c_uint,
                                     pubkey: *mut libc::c_uchar,
                                     pubsize: libc::c_uint)
     -> libc::c_int;
    pub fn Testsuite_Verify_Signature(arg1: TSS_HCONTEXT, arg2: TSS_HKEY,
                                      arg3: *mut TSS_VALIDATION)
     -> TSS_RESULT;
    pub fn Testsuite_Is_Ordinal_Supported(arg1: TSS_HTPM,
                                          arg2: TPM_COMMAND_CODE)
     -> TSS_RESULT;
    pub fn main_v1_1() -> libc::c_int;
    pub fn main_v1_2(arg1: libc::c_char) -> libc::c_int;
}
