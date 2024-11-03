use ::libc;
extern "C" {
	fn free(_: *mut libc::c_void);
	fn arr_len(arr: *const *mut libc::c_char) -> size_t;
	fn equal(expected: *const libc::c_char, actual: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
// #[no_mangle]
// pub unsafe extern "C" fn rm_str_arr(
//     mut arr: *mut *mut libc::c_char,
//     mut s: *const libc::c_char,
// ) {
//     let mut i: size_t = 0;
//     let mut len: size_t = 0;
//     if arr.is_null() || s.is_null() {
//         return;
//     }
//     i = 0 as libc::c_int as size_t;
//     len = arr_len(arr);
//     while !(*arr.offset(i as isize)).is_null() {
//         if !(equal(*arr.offset(i as isize), s)).is_null() {
//             free(*arr.offset(i as isize) as *mut libc::c_void);
//             while i < len {
//                 let ref mut fresh0 = *arr.offset(i as isize);
//                 *fresh0 = *arr
//                     .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
//                 i = i.wrapping_add(1);
//                 i;
//             }
//             return;
//         }
//         i = i.wrapping_add(1);
//         i;
//     }
// }
