#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: uint32_t) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_opt {
    pub rule: *mut bt_node,
    /* _OTL_OPT_H_ */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bt_node {
    pub flag: libc::c_int,
    pub left: *mut bt_node,
    pub right: *mut bt_node,
    pub data: [libc::c_char; 4],
}
unsafe extern "C" fn match_expr(
    mut expr: *mut bt_node,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut retval: libc::c_int = 1i32;
    let mut i: libc::c_int = 0;
    if !expr.is_null() {
        if (*expr).left.is_null() && (*expr).right.is_null() {
            i = 0i32;
            while i < 4i32 {
                if (*expr).data[i as usize] as libc::c_int != '?' as i32
                    && (*expr).data[i as usize] as libc::c_int
                        != *key.offset(i as isize) as libc::c_int
                {
                    retval = 0i32;
                    break;
                } else {
                    i += 1
                }
            }
        } else {
            if !(*expr).left.is_null() {
                retval = match_expr((*expr).left, key)
            }
            if !(*expr).right.is_null() {
                if retval != 0 && (*expr).flag & 1i32 << 1i32 != 0 {
                    retval &= match_expr((*expr).right, key)
                } else if retval == 0 && (*expr).flag & 1i32 << 1i32 == 0 {
                    retval = match_expr((*expr).right, key)
                }
            }
        }
        if (*expr).flag & 1i32 << 0i32 != 0 {
            retval = if retval != 0 { 0i32 } else { 1i32 }
        }
    }
    return retval;
}
unsafe extern "C" fn bt_new_tree() -> *mut bt_node {
    let mut expr: *mut bt_node = 0 as *mut bt_node;
    expr = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<bt_node>() as libc::c_ulong) as uint32_t)
        as *mut bt_node;
    (*expr).flag = 0i32;
    (*expr).left = 0 as *mut bt_node;
    (*expr).right = 0 as *mut bt_node;
    memset(
        (*expr).data.as_mut_ptr() as *mut libc::c_void,
        0i32,
        4i32 as libc::c_ulong,
    );
    return expr;
}
unsafe extern "C" fn bt_release_tree(mut tree: *mut bt_node) {
    if !tree.is_null() {
        if !(*tree).left.is_null() {
            bt_release_tree((*tree).left);
        }
        if !(*tree).right.is_null() {
            bt_release_tree((*tree).right);
        }
        free(tree as *mut libc::c_void);
    };
}
unsafe extern "C" fn parse_expr(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut bt_node {
    let mut root: *mut bt_node = 0 as *mut bt_node;
    let mut curr: *mut bt_node = 0 as *mut bt_node;
    if *pp >= endptr {
        return 0 as *mut bt_node;
    }
    curr = bt_new_tree();
    root = curr;
    while *pp < endptr {
        match **pp as libc::c_int {
            33 => {
                if (*curr).flag & 2i32 != 0 {
                    (*curr).flag &= !(1i32 << 0i32)
                } else {
                    (*curr).flag |= 1i32 << 0i32
                }
                *pp = (*pp).offset(1)
            }
            40 => {
                *pp = (*pp).offset(1);
                if *pp < endptr {
                    let mut expr: *mut bt_node = 0 as *mut bt_node;
                    expr = parse_expr(pp, endptr);
                    if expr.is_null() {
                        dpx_warning(
                            b"Syntax error: %s\n\x00" as *const u8 as *const libc::c_char,
                            *pp,
                        );
                        return 0 as *mut bt_node;
                    }
                    if **pp as libc::c_int != ')' as i32 {
                        dpx_warning(
                            b"Syntax error: Unbalanced ()\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return 0 as *mut bt_node;
                    }
                    (*curr).left = (*expr).left;
                    (*curr).right = (*expr).right;
                    memcpy(
                        (*curr).data.as_mut_ptr() as *mut libc::c_void,
                        (*expr).data.as_mut_ptr() as *const libc::c_void,
                        4i32 as libc::c_ulong,
                    );
                    free(expr as *mut libc::c_void);
                } else {
                    dpx_warning(
                        b"Syntax error: Unbalanced ()\n\x00" as *const u8 as *const libc::c_char,
                    );
                    bt_release_tree(root);
                    return 0 as *mut bt_node;
                }
                *pp = (*pp).offset(1)
            }
            41 => return root,
            124 | 38 => {
                if *pp >= endptr {
                    dpx_warning(
                        b"Syntax error: %s\n\x00" as *const u8 as *const libc::c_char,
                        *pp,
                    );
                    bt_release_tree(root);
                    return 0 as *mut bt_node;
                } else {
                    let mut tmp: *mut bt_node = 0 as *mut bt_node;
                    tmp = bt_new_tree();
                    (*tmp).left = root;
                    curr = bt_new_tree();
                    (*tmp).right = curr;
                    if **pp as libc::c_int == '&' as i32 {
                        (*tmp).flag = 1i32
                    } else {
                        (*tmp).flag = 0i32
                    }
                    root = tmp
                }
                *pp = (*pp).offset(1)
            }
            42 => {
                memset(
                    (*curr).data.as_mut_ptr() as *mut libc::c_void,
                    '?' as i32,
                    4i32 as libc::c_ulong,
                );
                *pp = (*pp).offset(1)
            }
            _ => {
                if (*pp).offset(4) <= endptr {
                    let mut i: libc::c_int = 0;
                    i = 0i32;
                    while i < 4i32 {
                        if **pp as libc::c_int == ' ' as i32
                            || **pp as libc::c_int == '?' as i32
                            || *(*__ctype_b_loc())
                                .offset(**pp as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            || *(*__ctype_b_loc())
                                .offset(**pp as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            (*curr).data[i as usize] = **pp
                        } else if **pp as libc::c_int == '_' as i32 {
                            (*curr).data[i as usize] = ' ' as i32 as libc::c_char
                        } else {
                            dpx_warning(
                                b"Invalid char in tag: %c\n\x00" as *const u8
                                    as *const libc::c_char,
                                **pp as libc::c_int,
                            );
                            bt_release_tree(root);
                            return 0 as *mut bt_node;
                        }
                        *pp = (*pp).offset(1);
                        i += 1
                    }
                } else {
                    dpx_warning(
                        b"Syntax error: %s\n\x00" as *const u8 as *const libc::c_char,
                        *pp,
                    );
                    bt_release_tree(root);
                    return 0 as *mut bt_node;
                }
            }
        }
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn otl_new_opt() -> *mut otl_opt {
    let mut opt: *mut otl_opt = 0 as *mut otl_opt;
    opt = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<otl_opt>() as libc::c_ulong) as uint32_t)
        as *mut otl_opt;
    (*opt).rule = 0 as *mut bt_node;
    return opt as *mut otl_opt;
}
#[no_mangle]
pub unsafe extern "C" fn otl_release_opt(mut opt: *mut otl_opt) {
    if !(*opt).rule.is_null() {
        bt_release_tree((*opt).rule);
    }
    (*opt).rule = 0 as *mut bt_node;
    free(opt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otl_parse_optstring(
    mut opt: *mut otl_opt,
    mut optstr: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    if !opt.is_null() {
    } else {
        __assert_fail(
            b"opt\x00" as *const u8 as *const libc::c_char,
            b"dpx-otl_opt.c\x00" as *const u8 as *const libc::c_char,
            237i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"int otl_parse_optstring(otl_opt *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !optstr.is_null() {
        p = optstr;
        endptr = p.offset(strlen(optstr) as isize);
        (*opt).rule = parse_expr(&mut p, endptr)
    }
    return 0i32;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#[no_mangle]
pub unsafe extern "C" fn otl_match_optrule(
    mut opt: *mut otl_opt,
    mut tag: *const libc::c_char,
) -> libc::c_int {
    if !tag.is_null() {
    } else {
        __assert_fail(
            b"tag\x00" as *const u8 as *const libc::c_char,
            b"dpx-otl_opt.c\x00" as *const u8 as *const libc::c_char,
            251i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int otl_match_optrule(otl_opt *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if opt.is_null() || (*opt).rule.is_null() {
        return 1i32;
    }
    return match_expr((*opt).rule, tag);
}