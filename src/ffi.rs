use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};
use std::ptr;
use std::slice;

use rustc_hash::FxHashMap as HashMap;

use crate::encoding;
use crate::{CoreBPE, Rank, _byte_pair_merge, byte_pair_encode as bpe_encode, byte_pair_split as bpe_split};

#[repr(C)]
pub struct CoreBPEError {
    pub message: *mut c_char,
    pub is_key_error: bool,
}

#[repr(C)]
pub struct EncodeResult {
    pub tokens: *mut c_uint,
    pub len: usize,
    pub last_piece_token_len: usize,
}

#[repr(C)]
pub struct CoreBPEConfig {
    pub encoder_keys: *const *const u8,
    pub encoder_key_lengths: *const usize,
    pub encoder_values: *const c_uint,
    pub encoder_len: usize,
    pub special_tokens_keys: *const *const c_char,
    pub special_tokens_values: *const c_uint,
    pub special_tokens_len: usize,
    pub pattern: *const c_char,
}

#[repr(C)]
pub struct UnstableEncodeResult {
    pub tokens: *mut c_uint,
    pub tokens_len: usize,
    pub completions: *mut *mut c_uint,
    pub completions_lens: *mut usize,
    pub completions_count: usize,
}

pub struct CoreBPEHandle {
    inner: CoreBPE,
}

fn set_error(error: *mut CoreBPEError, message: String, is_key_error: bool) {
    if error.is_null() {
        return;
    }
    unsafe {
        (*error).message = string_to_c(message);
        (*error).is_key_error = is_key_error;
    }
}

fn clear_error(error: *mut CoreBPEError) {
    if error.is_null() {
        return;
    }
    unsafe {
        (*error).message = ptr::null_mut();
        (*error).is_key_error = false;
    }
}

fn string_to_c(s: String) -> *mut c_char {
    CString::new(s)
        .map(|c| c.into_raw())
        .unwrap_or(ptr::null_mut())
}

fn vec_u32_to_c(vec: Vec<Rank>) -> (*mut c_uint, usize) {
    let mut vec = vec;
    let len = vec.len();
    let ptr = vec.as_mut_ptr() as *mut c_uint;
    std::mem::forget(vec);
    (ptr, len)
}

fn ranks_from_config(config: &CoreBPEConfig) -> Result<CoreBPE, String> {
    if config.encoder_len == 0 {
        return Err("encoder_len must be > 0".into());
    }
    if config.encoder_keys.is_null()
        || config.encoder_key_lengths.is_null()
        || config.encoder_values.is_null()
    {
        return Err("encoder arrays must not be null".into());
    }
    if config.pattern.is_null() {
        return Err("pattern must not be null".into());
    }

    unsafe {
        let keys = slice::from_raw_parts(config.encoder_keys, config.encoder_len);
        let key_lengths = slice::from_raw_parts(config.encoder_key_lengths, config.encoder_len);
        let values = slice::from_raw_parts(config.encoder_values, config.encoder_len);

        let mut encoder = HashMap::default();
        for i in 0..config.encoder_len {
            let key = slice::from_raw_parts(keys[i], key_lengths[i]).to_vec();
            encoder.insert(key, values[i]);
        }

        let mut special_tokens = HashMap::default();
        if config.special_tokens_len > 0 {
            if config.special_tokens_keys.is_null() || config.special_tokens_values.is_null() {
                return Err("special token arrays must not be null".into());
            }
            let special_keys =
                slice::from_raw_parts(config.special_tokens_keys, config.special_tokens_len);
            let special_values =
                slice::from_raw_parts(config.special_tokens_values, config.special_tokens_len);
            for i in 0..config.special_tokens_len {
                let key = CStr::from_ptr(special_keys[i])
                    .to_str()
                    .map_err(|e| e.to_string())?
                    .to_string();
                special_tokens.insert(key, special_values[i]);
            }
        }

        let pattern = CStr::from_ptr(config.pattern)
            .to_str()
            .map_err(|e| e.to_string())?;

        CoreBPE::new(encoder, special_tokens, pattern).map_err(|e| e.to_string())
    }
}

fn allowed_special_from_c(
    allowed_special: *const *const c_char,
    allowed_special_len: usize,
) -> Result<(HashSet<&str>, Vec<String>), String> {
    if allowed_special_len == 0 {
        return Ok((HashSet::new(), Vec::new()));
    }
    if allowed_special.is_null() {
        return Err("allowed_special must not be null when len > 0".into());
    }

    let mut owned = Vec::with_capacity(allowed_special_len);
    unsafe {
        let ptrs = slice::from_raw_parts(allowed_special, allowed_special_len);
        for &ptr in ptrs {
            owned.push(
                CStr::from_ptr(ptr)
                    .to_str()
                    .map_err(|e| e.to_string())?
                    .to_string(),
            );
        }
    }
    let set = owned.iter().map(|s| s.as_str()).collect();
    Ok((set, owned))
}

fn ranks_from_c_arrays(
    ranks_keys: *const *const u8,
    ranks_key_lengths: *const usize,
    ranks_values: *const c_uint,
    ranks_len: usize,
) -> Result<HashMap<Vec<u8>, Rank>, String> {
    if ranks_len == 0 {
        return Err("ranks_len must be > 0".into());
    }
    if ranks_keys.is_null() || ranks_key_lengths.is_null() || ranks_values.is_null() {
        return Err("rank arrays must not be null".into());
    }

    unsafe {
        let keys = slice::from_raw_parts(ranks_keys, ranks_len);
        let key_lengths = slice::from_raw_parts(ranks_key_lengths, ranks_len);
        let values = slice::from_raw_parts(ranks_values, ranks_len);
        let mut ranks = HashMap::default();
        for i in 0..ranks_len {
            let key = slice::from_raw_parts(keys[i], key_lengths[i]).to_vec();
            ranks.insert(key, values[i]);
        }
        Ok(ranks)
    }
}

#[no_mangle]
pub extern "C" fn tiktoken_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn tiktoken_list_encoding_names(out_len: *mut usize) -> *mut *mut c_char {
    if out_len.is_null() {
        return ptr::null_mut();
    }
    let names = encoding::list_encoding_names();
    unsafe {
        *out_len = names.len();
    }
    let mut c_strings: Vec<*mut c_char> = names
        .into_iter()
        .map(|name| string_to_c(name.to_string()))
        .collect();
    let ptr = c_strings.as_mut_ptr();
    std::mem::forget(c_strings);
    ptr
}

#[no_mangle]
pub extern "C" fn tiktoken_get_encoding(
    name: *const c_char,
    error: *mut CoreBPEError,
) -> *mut CoreBPEHandle {
    clear_error(error);
    if name.is_null() {
        set_error(error, "name must not be null".into(), false);
        return ptr::null_mut();
    }

    let name = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(error, e.to_string(), false);
                return ptr::null_mut();
            }
        }
    };

    match encoding::get_encoding(name) {
        Ok(core) => Box::into_raw(Box::new(CoreBPEHandle { inner: core })),
        Err(e) => {
            set_error(error, e.to_string(), false);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn corebpe_new(
    config: *const CoreBPEConfig,
    error: *mut CoreBPEError,
) -> *mut CoreBPEHandle {
    clear_error(error);
    if config.is_null() {
        set_error(error, "config must not be null".into(), false);
        return ptr::null_mut();
    }

    match ranks_from_config(unsafe { &*config }) {
        Ok(core) => Box::into_raw(Box::new(CoreBPEHandle { inner: core })),
        Err(e) => {
            set_error(error, e, false);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn corebpe_free(bpe: *mut CoreBPEHandle) {
    if bpe.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(bpe));
    }
}

#[no_mangle]
pub extern "C" fn corebpe_error_free(error: *mut CoreBPEError) {
    if error.is_null() {
        return;
    }
    unsafe {
        if !(*error).message.is_null() {
            drop(CString::from_raw((*error).message));
            (*error).message = ptr::null_mut();
        }
        (*error).is_key_error = false;
    }
}

#[no_mangle]
pub extern "C" fn corebpe_encode_ordinary(bpe: *mut CoreBPEHandle, text: *const c_char) -> EncodeResult {
    let empty = EncodeResult {
        tokens: ptr::null_mut(),
        len: 0,
        last_piece_token_len: 0,
    };
    if bpe.is_null() || text.is_null() {
        return empty;
    }

    let text = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return empty,
        }
    };

    let tokens = unsafe { (&(*bpe).inner).encode_ordinary(text) };
    let (ptr, len) = vec_u32_to_c(tokens);
    EncodeResult {
        tokens: ptr,
        len,
        last_piece_token_len: 0,
    }
}

#[no_mangle]
pub extern "C" fn corebpe_encode(
    bpe: *mut CoreBPEHandle,
    text: *const c_char,
    allowed_special: *const *const c_char,
    allowed_special_len: usize,
    error: *mut CoreBPEError,
) -> EncodeResult {
    let empty = EncodeResult {
        tokens: ptr::null_mut(),
        len: 0,
        last_piece_token_len: 0,
    };
    clear_error(error);
    if bpe.is_null() || text.is_null() {
        set_error(error, "bpe and text must not be null".into(), false);
        return empty;
    }

    let text = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(error, e.to_string(), false);
                return empty;
            }
        }
    };

    let (allowed, _owned) = match allowed_special_from_c(allowed_special, allowed_special_len) {
        Ok(v) => v,
        Err(e) => {
            set_error(error, e, false);
            return empty;
        }
    };

    match unsafe { (&(*bpe).inner).encode(text, &allowed) } {
        Ok((tokens, last_piece_token_len)) => {
            let (ptr, len) = vec_u32_to_c(tokens);
            EncodeResult {
                tokens: ptr,
                len,
                last_piece_token_len,
            }
        }
        Err(e) => {
            set_error(error, e.message, false);
            empty
        }
    }
}

#[no_mangle]
pub extern "C" fn corebpe_encode_with_special_tokens(
    bpe: *mut CoreBPEHandle,
    text: *const c_char,
) -> EncodeResult {
    let empty = EncodeResult {
        tokens: ptr::null_mut(),
        len: 0,
        last_piece_token_len: 0,
    };
    if bpe.is_null() || text.is_null() {
        return empty;
    }

    let text = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return empty,
        }
    };

    let tokens = unsafe { (&(*bpe).inner).encode_with_special_tokens(text) };
    let (ptr, len) = vec_u32_to_c(tokens);
    EncodeResult {
        tokens: ptr,
        len,
        last_piece_token_len: 0,
    }
}

#[no_mangle]
pub extern "C" fn corebpe_encode_unstable_native(
    bpe: *mut CoreBPEHandle,
    text: *const c_char,
    allowed_special: *const *const c_char,
    allowed_special_len: usize,
) -> UnstableEncodeResult {
    let empty = UnstableEncodeResult {
        tokens: ptr::null_mut(),
        tokens_len: 0,
        completions: ptr::null_mut(),
        completions_lens: ptr::null_mut(),
        completions_count: 0,
    };
    if bpe.is_null() || text.is_null() {
        return empty;
    }

    let text = unsafe {
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return empty,
        }
    };

    let (allowed, _owned) =
        allowed_special_from_c(allowed_special, allowed_special_len).unwrap_or_default();
    let (tokens, completions) =
        unsafe { (&(*bpe).inner)._encode_unstable_native(text, &allowed) };

    let (tokens_ptr, tokens_len) = vec_u32_to_c(tokens);
    let completions_count = completions.len();
    let mut completion_vecs: Vec<Vec<Rank>> = completions.into_iter().collect();
    let mut completion_ptrs = Vec::with_capacity(completions_count);
    let mut completion_lens = Vec::with_capacity(completions_count);

    for completion in completion_vecs.drain(..) {
        let (ptr, len) = vec_u32_to_c(completion);
        completion_ptrs.push(ptr);
        completion_lens.push(len);
    }

    let completions_ptr = completion_ptrs.as_mut_ptr();
    let completions_lens_ptr = completion_lens.as_mut_ptr();
    std::mem::forget(completion_ptrs);
    std::mem::forget(completion_lens);

    UnstableEncodeResult {
        tokens: tokens_ptr,
        tokens_len,
        completions: completions_ptr,
        completions_lens: completions_lens_ptr,
        completions_count,
    }
}

#[no_mangle]
pub extern "C" fn corebpe_decode_bytes(
    bpe: *mut CoreBPEHandle,
    tokens: *const c_uint,
    tokens_len: usize,
    out_len: *mut usize,
    error: *mut CoreBPEError,
) -> *mut u8 {
    clear_error(error);
    if out_len.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        *out_len = 0;
    }
    if bpe.is_null() || tokens.is_null() {
        set_error(error, "bpe and tokens must not be null".into(), false);
        return ptr::null_mut();
    }

    let tokens: Vec<Rank> = unsafe {
        slice::from_raw_parts(tokens, tokens_len)
            .iter()
            .copied()
            .collect()
    };

    match unsafe { (&(*bpe).inner).decode_bytes(&tokens) } {
        Ok(bytes) => {
            let mut bytes = bytes;
            let len = bytes.len();
            let ptr = bytes.as_mut_ptr();
            std::mem::forget(bytes);
            unsafe {
                *out_len = len;
            }
            ptr
        }
        Err(e) => {
            set_error(error, e.to_string(), true);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn corebpe_decode(
    bpe: *mut CoreBPEHandle,
    tokens: *const c_uint,
    tokens_len: usize,
    error: *mut CoreBPEError,
) -> *mut c_char {
    clear_error(error);
    if bpe.is_null() || tokens.is_null() {
        set_error(error, "bpe and tokens must not be null".into(), false);
        return ptr::null_mut();
    }

    let tokens: Vec<Rank> = unsafe {
        slice::from_raw_parts(tokens, tokens_len)
            .iter()
            .copied()
            .collect()
    };

    match unsafe { (&(*bpe).inner).decode(&tokens) } {
        Ok(text) => string_to_c(text),
        Err(e) => {
            let is_key = e.message.contains("Invalid token");
            set_error(error, e.message, is_key);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn corebpe_special_tokens(
    bpe: *mut CoreBPEHandle,
    out_len: *mut usize,
) -> *mut *mut c_char {
    if bpe.is_null() || out_len.is_null() {
        return ptr::null_mut();
    }

    let specials = unsafe { (&(*bpe).inner).special_tokens() };
    unsafe {
        *out_len = specials.len();
    }

    let mut strings: Vec<*mut c_char> = specials
        .into_iter()
        .map(|s| string_to_c(s.to_string()))
        .collect();
    let ptr = strings.as_mut_ptr();
    std::mem::forget(strings);
    ptr
}

#[no_mangle]
pub extern "C" fn encode_result_free(result: EncodeResult) {
    if !result.tokens.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(result.tokens, result.len, result.len);
        }
    }
}

#[no_mangle]
pub extern "C" fn unstable_encode_result_free(result: UnstableEncodeResult) {
    if !result.tokens.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(result.tokens, result.tokens_len, result.tokens_len);
        }
    }
    if !result.completions.is_null() && !result.completions_lens.is_null() {
        unsafe {
            let ptrs = slice::from_raw_parts(result.completions, result.completions_count);
            let lens = slice::from_raw_parts(result.completions_lens, result.completions_count);
            for i in 0..result.completions_count {
                if !ptrs[i].is_null() {
                    let _ = Vec::from_raw_parts(ptrs[i], lens[i], lens[i]);
                }
            }
            let _ = Vec::from_raw_parts(
                result.completions,
                result.completions_count,
                result.completions_count,
            );
            let _ = Vec::from_raw_parts(
                result.completions_lens,
                result.completions_count,
                result.completions_count,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn strings_array_free(array: *mut *mut c_char, len: usize) {
    if array.is_null() {
        return;
    }
    unsafe {
        let strings = Vec::from_raw_parts(array, len, len);
        for s in strings {
            if !s.is_null() {
                drop(CString::from_raw(s));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn byte_pair_merge(
    ranks_keys: *const *const u8,
    ranks_key_lengths: *const usize,
    ranks_values: *const c_uint,
    ranks_len: usize,
    piece: *const u8,
    piece_len: usize,
    out_len: *mut usize,
) -> *mut usize {
    if out_len.is_null() || piece.is_null() {
        return ptr::null_mut();
    }

    let ranks = match ranks_from_c_arrays(ranks_keys, ranks_key_lengths, ranks_values, ranks_len) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };
    let piece = unsafe { slice::from_raw_parts(piece, piece_len) };
    let merged = _byte_pair_merge(&ranks, piece);
    let mut starts: Vec<usize> = merged.into_iter().map(|(start, _)| start).collect();
    let len = starts.len();
    let ptr = starts.as_mut_ptr();
    std::mem::forget(starts);
    unsafe {
        *out_len = len;
    }
    ptr
}

#[no_mangle]
pub extern "C" fn byte_pair_encode(
    ranks_keys: *const *const u8,
    ranks_key_lengths: *const usize,
    ranks_values: *const c_uint,
    ranks_len: usize,
    piece: *const u8,
    piece_len: usize,
    out_len: *mut usize,
) -> *mut c_uint {
    if out_len.is_null() || piece.is_null() {
        return ptr::null_mut();
    }

    let ranks = match ranks_from_c_arrays(ranks_keys, ranks_key_lengths, ranks_values, ranks_len) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };
    let piece = unsafe { slice::from_raw_parts(piece, piece_len) };
    let encoded = bpe_encode(piece, &ranks);
    let (ptr, len) = vec_u32_to_c(encoded);
    unsafe {
        *out_len = len;
    }
    ptr
}

#[no_mangle]
pub extern "C" fn byte_pair_split(
    ranks_keys: *const *const u8,
    ranks_key_lengths: *const usize,
    ranks_values: *const c_uint,
    ranks_len: usize,
    piece: *const u8,
    piece_len: usize,
    out_len: *mut usize,
    out_sublens: *mut *mut usize,
) -> *mut *mut u8 {
    if out_len.is_null() || piece.is_null() || piece_len <= 1 {
        return ptr::null_mut();
    }
    unsafe {
        *out_len = 0;
        if !out_sublens.is_null() {
            *out_sublens = ptr::null_mut();
        }
    }

    let ranks = match ranks_from_c_arrays(ranks_keys, ranks_key_lengths, ranks_values, ranks_len) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };
    let piece = unsafe { slice::from_raw_parts(piece, piece_len) };
    let splits = bpe_split(piece, &ranks);

    let mut owned: Vec<Vec<u8>> = splits.into_iter().map(|s| s.to_vec()).collect();
    let mut lens: Vec<usize> = owned.iter().map(|v| v.len()).collect();
    let mut ptrs: Vec<*mut u8> = owned.iter_mut().map(|v| v.as_mut_ptr()).collect();
    let count = ptrs.len();
    let out_ptr = ptrs.as_mut_ptr();
    std::mem::forget(owned);
    std::mem::forget(ptrs);

    if !out_sublens.is_null() {
        let lens_ptr = lens.as_mut_ptr();
        std::mem::forget(lens);
        unsafe {
            *out_sublens = lens_ptr;
        }
    } else {
        std::mem::forget(lens);
    }

    unsafe {
        *out_len = count;
    }
    out_ptr
}

#[no_mangle]
pub extern "C" fn byte_pair_split_free(
    pieces: *mut *mut u8,
    piece_lens: *mut usize,
    count: usize,
) {
    if pieces.is_null() || piece_lens.is_null() {
        return;
    }
    unsafe {
        let ptrs = Vec::from_raw_parts(pieces, count, count);
        let lens = Vec::from_raw_parts(piece_lens, count, count);
        for i in 0..count {
            if !ptrs[i].is_null() {
                let _ = Vec::from_raw_parts(ptrs[i], lens[i], lens[i]);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn tiktoken_free_bytes(ptr: *mut u8, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

#[no_mangle]
pub extern "C" fn tiktoken_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}

#[no_mangle]
pub extern "C" fn tiktoken_free_u32_array(ptr: *mut c_uint, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

#[no_mangle]
pub extern "C" fn tiktoken_free_usize_array(ptr: *mut usize, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}
