/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use std::slice;
use trash::delete;
use trash::delete_all;

/// Delete a file.
/// 
/// Example:
/// * *
/// int main()
/// {
///   if (trash_delete("file.txt") != 0)
///   {
///     printf("Something went wrong\n");
///     return -1;
///   }
///   return 0;
/// }
/// * *
/// 
/// @param path
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn trash_delete(path: *const c_char) -> c_int {
  if path.is_null() {
    return -1;
  }
  let str = match CStr::from_ptr(path).to_str() {
    Ok(s) => s,
    Err(_) => return -1
  };
  match delete(str) {
    Ok(_) => 0,
    Err(_) => -1
  }
}

/// Delete multiple files.
/// 
/// Example:
/// * *
/// int main()
/// {
///   const char* arr[] = { "file.txt", "file2.txt" };
///   if (trash_delete_all(arr, 2) != 0)
///   {
///     printf("Something went wrong\n");
///     return -1;
///   }
///   return 0;
/// }
/// * *
/// 
/// @param path
/// @param length
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn trash_delete_all(paths: *const *const c_char, length: usize) -> c_int {
  if paths.is_null() {
    return -1;
  }
  let array = slice::from_raw_parts(paths, length);
  let vec: Vec<String> = array.iter().map(|&i| CStr::from_ptr(i).to_string_lossy().into_owned()).collect();
  match delete_all(vec) {
    Ok(_) => 0,
    Err(_) => -1
  }
}