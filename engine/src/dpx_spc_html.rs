#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn round(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_null() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_boolean(value: libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut libc::c_void;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const libc::c_char) -> *mut pdf_obj;
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn spc_begin_annot(spe: *mut spc_env, annot_dict: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn spc_end_annot(spe: *mut spc_env) -> libc::c_int;
    #[no_mangle]
    fn parse_float_decimal(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn parse_c_ident(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    fn pdf_doc_get_reference(category: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    /* Not really managing tree...
     * There should be something for number tree.
     */
    #[no_mangle]
    fn pdf_doc_add_names(
        category: *const libc::c_char,
        key: *const libc::c_void,
        keylen: libc::c_int,
        value: *mut pdf_obj,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const libc::c_char, length: libc::c_uint);
    #[no_mangle]
    fn pdf_doc_add_page_resource(
        category: *const libc::c_char,
        resource_name: *const libc::c_char,
        resources: *mut pdf_obj,
    );
    #[no_mangle]
    fn pdf_dev_rectclip(
        x: libc::c_double,
        y: libc::c_double,
        w: libc::c_double,
        h: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[no_mangle]
    fn pdf_dev_gsave() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_grestore() -> libc::c_int;
    #[no_mangle]
    fn pdf_ximage_get_resname(xobj_id: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_ximage_get_reference(xobj_id: libc::c_int) -> *mut pdf_obj;
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const libc::c_char, options: load_options) -> libc::c_int;
    #[no_mangle]
    fn pdf_ximage_scale_image(
        id: libc::c_int,
        M: *mut pdf_tmatrix,
        r: *mut pdf_rect,
        p: *mut transform_info,
    ) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: libc::c_double,
    pub y_user: libc::c_double,
    pub mag: libc::c_double,
    pub pg: libc::c_int,
    /* current page in PDF */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const libc::c_char,
    pub endptr: *const libc::c_char,
    pub base: *const libc::c_char,
    pub command: *const libc::c_char,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const libc::c_char,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_html_ {
    pub opts: C2RustUnnamed_0,
    pub link_dict: *mut pdf_obj,
    pub baseurl: *mut libc::c_char,
    pub pending_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub extensions: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub f: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub depth: libc::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: libc::c_int,
    pub bbox_type: libc::c_int,
    pub dict: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
static mut _html_state: spc_html_ = {
    let mut init = spc_html_ {
        opts: {
            let mut init = C2RustUnnamed_0 { extensions: 0i32 };
            init
        },
        link_dict: 0 as *const pdf_obj as *mut pdf_obj,
        baseurl: 0 as *const libc::c_char as *mut libc::c_char,
        pending_type: -1i32,
    };
    init
};
/* ENABLE_HTML_SVG_TRANSFORM */
unsafe extern "C" fn parse_key_val(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut kp: *mut *mut libc::c_char,
    mut vp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut q: *const libc::c_char = 0 as *const libc::c_char; /* skip '="' */
                                                               
    let mut p: *const libc::c_char = 0 as *const libc::c_char; /* include trailing NULL here!!! */
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char; /* we may want to add '/' */
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char; /* Should be checked somewhere else */
    let mut n: libc::c_int = 0; /* Assume this is URL */
    let mut error: libc::c_int = 0i32;
    p = *pp;
    while p < endptr
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    v = 0 as *mut libc::c_char;
    k = v;
    q = p;
    n = 0i32;
    while p < endptr
        && (*p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
            || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32
            || *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32
            || *p as libc::c_int == '-' as i32
            || *p as libc::c_int == ':' as i32)
    {
        n += 1;
        p = p.offset(1)
    }
    if n == 0i32 {
        *vp = 0 as *mut libc::c_char;
        *kp = *vp;
        return -1i32;
    }
    k = new(((n + 1i32) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    memcpy(
        k as *mut libc::c_void,
        q as *const libc::c_void,
        n as libc::c_ulong,
    );
    *k.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    if p.offset(2) >= endptr
        || *p.offset(0) as libc::c_int != '=' as i32
        || *p.offset(1) as libc::c_int != '\"' as i32 && *p.offset(1) as libc::c_int != '\'' as i32
    {
        k = mfree(k as *mut libc::c_void) as *mut libc::c_char;
        *pp = p;
        error = -1i32
    } else {
        let mut qchr: libc::c_char = *p.offset(1);
        p = p.offset(2);
        q = p;
        n = 0i32;
        while p < endptr && *p as libc::c_int != qchr as libc::c_int {
            p = p.offset(1);
            n += 1
        }
        if p == endptr || *p as libc::c_int != qchr as libc::c_int {
            error = -1i32
        } else {
            v = new(((n + 1i32) as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as uint32_t) as *mut libc::c_char;
            memcpy(
                v as *mut libc::c_void,
                q as *const libc::c_void,
                n as libc::c_ulong,
            );
            *v.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
            p = p.offset(1)
        }
    }
    *kp = k;
    *vp = v;
    *pp = p;
    return error;
}
unsafe extern "C" fn read_html_tag(
    mut name: *mut libc::c_char,
    mut attr: *mut pdf_obj,
    mut type_0: *mut libc::c_int,
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = *pp;
    let mut n: libc::c_int = 0i32;
    let mut error: libc::c_int = 0i32;
    while p < endptr
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    if p >= endptr || *p as libc::c_int != '<' as i32 {
        return -1i32;
    }
    *type_0 = 1i32;
    p = p.offset(1);
    while p < endptr
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    if p < endptr && *p as libc::c_int == '/' as i32 {
        *type_0 = 2i32;
        p = p.offset(1);
        while p < endptr
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
    }
    n = 0i32;
    while p < endptr
        && n < 127i32
        && !(*p as libc::c_int == '>' as i32
            || *p as libc::c_int == '/' as i32
            || *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
    {
        *name.offset(n as isize) = *p;
        n += 1;
        p = p.offset(1)
    }
    *name.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    if n == 0i32
        || p == endptr
        || !(*p as libc::c_int == '>' as i32
            || *p as libc::c_int == '/' as i32
            || *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
    {
        *pp = p;
        return -1i32;
    }
    while p < endptr
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    while p < endptr
        && error == 0
        && *p as libc::c_int != '/' as i32
        && *p as libc::c_int != '>' as i32
    {
        let mut kp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut vp: *mut libc::c_char = 0 as *mut libc::c_char;
        error = parse_key_val(&mut p, endptr, &mut kp, &mut vp);
        if error == 0 {
            if !kp.is_null() {
                let mut _p: *mut libc::c_char = kp;
                while *_p as libc::c_int != 0i32 {
                    if *_p as libc::c_int >= 'A' as i32 && *_p as libc::c_int <= 'Z' as i32 {
                        *_p = (*_p as libc::c_int - 'A' as i32 + 'a' as i32) as libc::c_char
                    }
                    _p = _p.offset(1)
                }
            }
            pdf_add_dict(
                attr,
                pdf_new_name(kp),
                pdf_new_string(
                    vp as *const libc::c_void,
                    strlen(vp).wrapping_add(1i32 as libc::c_ulong),
                ),
            );
            free(kp as *mut libc::c_void);
            free(vp as *mut libc::c_void);
        }
        while p < endptr
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
    }
    if error != 0 {
        *pp = p;
        return error;
    }
    if p < endptr && *p as libc::c_int == '/' as i32 {
        *type_0 = 1i32;
        p = p.offset(1);
        while p < endptr
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
    }
    if p == endptr || *p as libc::c_int != '>' as i32 {
        *pp = p;
        return -1i32;
    }
    p = p.offset(1);
    if !name.is_null() {
        let mut _p_0: *mut libc::c_char = name;
        while *_p_0 as libc::c_int != 0i32 {
            if *_p_0 as libc::c_int >= 'A' as i32 && *_p_0 as libc::c_int <= 'Z' as i32 {
                *_p_0 = (*_p_0 as libc::c_int - 'A' as i32 + 'a' as i32) as libc::c_char
            }
            _p_0 = _p_0.offset(1)
        }
    }
    *pp = p;
    return 0i32;
}
unsafe extern "C" fn spc_handler_html__init(mut dp: *mut libc::c_void) -> libc::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    (*sd).link_dict = 0 as *mut pdf_obj;
    (*sd).baseurl = 0 as *mut libc::c_char;
    (*sd).pending_type = -1i32;
    return 0i32;
}
unsafe extern "C" fn spc_handler_html__clean(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> libc::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    free((*sd).baseurl as *mut libc::c_void);
    if (*sd).pending_type >= 0i32 || !(*sd).link_dict.is_null() {
        spc_warn(
            spe,
            b"Unclosed html anchor found.\x00" as *const u8 as *const libc::c_char,
        );
    }
    pdf_release_obj((*sd).link_dict);
    (*sd).pending_type = -1i32;
    (*sd).baseurl = 0 as *mut libc::c_char;
    (*sd).link_dict = 0 as *mut pdf_obj;
    return 0i32;
}
unsafe extern "C" fn spc_handler_html__bophook(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> libc::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    if (*sd).pending_type >= 0i32 {
        spc_warn(
            spe,
            b"...html anchor continues from previous page processed...\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return 0i32;
}
unsafe extern "C" fn spc_handler_html__eophook(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> libc::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    if (*sd).pending_type >= 0i32 {
        spc_warn(
            spe,
            b"Unclosed html anchor at end-of-page!\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0i32;
}
unsafe extern "C" fn fqurl(
    mut baseurl: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0i32;
    len = strlen(name) as libc::c_int;
    if !baseurl.is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(strlen(baseurl).wrapping_add(1i32 as libc::c_ulong))
            as libc::c_int as libc::c_int
    }
    q = new(((len + 1i32) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    *q = '\u{0}' as i32 as libc::c_char;
    if !baseurl.is_null() && *baseurl.offset(0) as libc::c_int != 0 {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        strcpy(q, baseurl);
        p = q.offset(strlen(q) as isize).offset(-1);
        if *p as libc::c_int == '/' as i32 {
            *p = '\u{0}' as i32 as libc::c_char
        }
        if *name.offset(0) as libc::c_int != 0 && *name.offset(0) as libc::c_int != '/' as i32 {
            strcat(q, b"/\x00" as *const u8 as *const libc::c_char);
        }
    }
    strcat(q, name);
    return q;
}
unsafe extern "C" fn html_open_link(
    mut spe: *mut spc_env,
    mut name: *const libc::c_char,
    mut sd: *mut spc_html_,
) -> libc::c_int {
    let mut color: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    if !name.is_null() {
    } else {
        __assert_fail(
            b"name\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_html.c\x00" as *const u8 as *const libc::c_char,
            289i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"int html_open_link(struct spc_env *, const char *, struct spc_html_ *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*sd).link_dict.is_null() {
    } else {
        __assert_fail(
            b"sd->link_dict == NULL\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_html.c\x00" as *const u8 as *const libc::c_char,
            290i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"int html_open_link(struct spc_env *, const char *, struct spc_html_ *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*sd).link_dict = pdf_new_dict();
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"Annot\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"Link\x00" as *const u8 as *const libc::c_char),
    );
    color = pdf_new_array();
    pdf_add_array(color, pdf_new_number(0.0f64));
    pdf_add_array(color, pdf_new_number(0.0f64));
    pdf_add_array(color, pdf_new_number(1.0f64));
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"C\x00" as *const u8 as *const libc::c_char),
        color,
    );
    url = fqurl((*sd).baseurl, name);
    if *url.offset(0) as libc::c_int == '#' as i32 {
        /* url++; causes memory leak in free(url) */
        pdf_add_dict(
            (*sd).link_dict,
            pdf_new_name(b"Dest\x00" as *const u8 as *const libc::c_char),
            pdf_new_string(url.offset(1) as *const libc::c_void, strlen(url.offset(1))),
        ); /* Otherwise must be bug */
    } else {
        let mut action: *mut pdf_obj = pdf_new_dict();
        pdf_add_dict(
            action,
            pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"Action\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"S\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"URI\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"URI\x00" as *const u8 as *const libc::c_char),
            pdf_new_string(url as *const libc::c_void, strlen(url)),
        );
        pdf_add_dict(
            (*sd).link_dict,
            pdf_new_name(b"A\x00" as *const u8 as *const libc::c_char),
            pdf_link_obj(action),
        );
        pdf_release_obj(action);
    }
    free(url as *mut libc::c_void);
    spc_begin_annot(spe, (*sd).link_dict);
    (*sd).pending_type = 0i32;
    return 0i32;
}
unsafe extern "C" fn html_open_dest(
    mut spe: *mut spc_env,
    mut name: *const libc::c_char,
    mut sd: *mut spc_html_,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut page_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    cp.x = (*spe).x_user;
    cp.y = (*spe).y_user;
    pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
    page_ref = pdf_doc_get_reference(b"@THISPAGE\x00" as *const u8 as *const libc::c_char);
    if !page_ref.is_null() {
    } else {
        __assert_fail(
            b"page_ref\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_html.c\x00" as *const u8 as *const libc::c_char,
            346i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"int html_open_dest(struct spc_env *, const char *, struct spc_html_ *)\x00",
            ))
            .as_ptr(),
        );
    }
    array = pdf_new_array();
    pdf_add_array(array, page_ref);
    pdf_add_array(
        array,
        pdf_new_name(b"XYZ\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_array(array, pdf_new_null());
    pdf_add_array(array, pdf_new_number(cp.y + 24.0f64));
    pdf_add_array(array, pdf_new_null());
    error = pdf_doc_add_names(
        b"Dests\x00" as *const u8 as *const libc::c_char,
        name as *const libc::c_void,
        strlen(name) as libc::c_int,
        array,
    );
    if error != 0 {
        spc_warn(
            spe,
            b"Failed to add named destination: %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    (*sd).pending_type = 1i32;
    return error;
}
unsafe extern "C" fn spc_html__anchor_open(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
    mut sd: *mut spc_html_,
) -> libc::c_int {
    let mut href: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut name: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: libc::c_int = 0i32;
    if (*sd).pending_type >= 0i32 || !(*sd).link_dict.is_null() {
        spc_warn(
            spe,
            b"Nested html anchors found!\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    href = pdf_lookup_dict(attr, b"href\x00" as *const u8 as *const libc::c_char);
    name = pdf_lookup_dict(attr, b"name\x00" as *const u8 as *const libc::c_char);
    if !href.is_null() && !name.is_null() {
        spc_warn(
            spe,
            b"Sorry, you can\'t have both \"href\" and \"name\" in anchor tag...\x00" as *const u8
                as *const libc::c_char,
        );
        error = -1i32
    } else if !href.is_null() {
        error = html_open_link(spe, pdf_string_value(href) as *const libc::c_char, sd)
    } else if !name.is_null() {
        /* name */
        error = html_open_dest(spe, pdf_string_value(name) as *const libc::c_char, sd)
    } else {
        spc_warn(
            spe,
            b"You should have \"href\" or \"name\" in anchor tag!\x00" as *const u8
                as *const libc::c_char,
        );
        error = -1i32
    }
    return error;
}
unsafe extern "C" fn spc_html__anchor_close(
    mut spe: *mut spc_env,
    mut sd: *mut spc_html_,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    match (*sd).pending_type {
        0 => {
            if !(*sd).link_dict.is_null() {
                spc_end_annot(spe);
                pdf_release_obj((*sd).link_dict);
                (*sd).link_dict = 0 as *mut pdf_obj;
                (*sd).pending_type = -1i32
            } else {
                spc_warn(
                    spe,
                    b"Closing html anchor (link) without starting!\x00" as *const u8
                        as *const libc::c_char,
                );
                error = -1i32
            }
        }
        1 => (*sd).pending_type = -1i32,
        _ => {
            spc_warn(
                spe,
                b"No corresponding opening tag for html anchor.\x00" as *const u8
                    as *const libc::c_char,
            );
            error = -1i32
        }
    }
    return error;
}
unsafe extern "C" fn spc_html__base_empty(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
    mut sd: *mut spc_html_,
) -> libc::c_int {
    let mut href: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut vp: *mut libc::c_char = 0 as *mut libc::c_char;
    href = pdf_lookup_dict(attr, b"href\x00" as *const u8 as *const libc::c_char);
    if href.is_null() {
        spc_warn(
            spe,
            b"\"href\" not found for \"base\" tag!\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    vp = pdf_string_value(href) as *mut libc::c_char;
    if !(*sd).baseurl.is_null() {
        spc_warn(
            spe,
            b"\"baseurl\" changed: \"%s\" --> \"%s\"\x00" as *const u8 as *const libc::c_char,
            (*sd).baseurl,
            vp,
        );
        free((*sd).baseurl as *mut libc::c_void);
    }
    (*sd).baseurl = new((strlen(vp).wrapping_add(1i32 as libc::c_ulong) as uint32_t
        as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    strcpy((*sd).baseurl, vp);
    return 0i32;
}
/* This isn't completed.
 * Please think about placement of images.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn atopt(mut a: *const libc::c_char) -> libc::c_double {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = a;
    let mut v: libc::c_double = 0.;
    let mut u: libc::c_double = 1.0f64;
    let mut _ukeys: [*const libc::c_char; 11] = [
        b"pt\x00" as *const u8 as *const libc::c_char,
        b"in\x00" as *const u8 as *const libc::c_char,
        b"cm\x00" as *const u8 as *const libc::c_char,
        b"mm\x00" as *const u8 as *const libc::c_char,
        b"bp\x00" as *const u8 as *const libc::c_char,
        b"pc\x00" as *const u8 as *const libc::c_char,
        b"dd\x00" as *const u8 as *const libc::c_char,
        b"cc\x00" as *const u8 as *const libc::c_char,
        b"sp\x00" as *const u8 as *const libc::c_char,
        b"px\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut k: libc::c_int = 0;
    q = parse_float_decimal(&mut p, p.offset(strlen(p) as isize));
    if q.is_null() {
        dpx_warning(
            b"Invalid length value: %s (%c)\x00" as *const u8 as *const libc::c_char,
            a,
            *p as libc::c_int,
        );
        return 0.0f64;
    }
    v = atof(q);
    free(q as *mut libc::c_void);
    q = parse_c_ident(&mut p, p.offset(strlen(p) as isize));
    if !q.is_null() {
        k = 0i32;
        while !_ukeys[k as usize].is_null() && strcmp(_ukeys[k as usize], q) != 0 {
            k += 1
        }
        match k {
            0 => u *= 72.0f64 / 72.27f64,
            1 => u *= 72.0f64,
            2 => u *= 72.0f64 / 2.54f64,
            3 => u *= 72.0f64 / 25.4f64,
            4 => u *= 1.0f64,
            5 => u *= 12.0f64 * 72.0f64 / 72.27f64,
            6 => u *= 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
            7 => u *= 12.0f64 * 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
            8 => u *= 72.0f64 / (72.27f64 * 65536i32 as libc::c_double),
            9 => u *= 1.0f64,
            _ => {
                dpx_warning(
                    b"Unknown unit of measure: %s\x00" as *const u8 as *const libc::c_char,
                    q,
                );
            }
        }
        free(q as *mut libc::c_void);
    }
    return v * u;
}
/* Replicated from spc_tpic */
unsafe extern "C" fn create_xgstate(mut a: libc::c_double, mut f_ais: libc::c_int) -> *mut pdf_obj
/* alpha is shape */ {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    dict = pdf_new_dict();
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"ExtGState\x00" as *const u8 as *const libc::c_char),
    );
    if f_ais != 0 {
        pdf_add_dict(
            dict,
            pdf_new_name(b"AIS\x00" as *const u8 as *const libc::c_char),
            pdf_new_boolean(1i32 as libc::c_char),
        );
    }
    pdf_add_dict(
        dict,
        pdf_new_name(b"ca\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(a),
    );
    return dict;
}
unsafe extern "C" fn check_resourcestatus(
    mut category: *const libc::c_char,
    mut resname: *const libc::c_char,
) -> libc::c_int {
    let mut dict1: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict2: *mut pdf_obj = 0 as *mut pdf_obj;
    dict1 = pdf_doc_current_page_resources();
    if dict1.is_null() {
        return 0i32;
    }
    dict2 = pdf_lookup_dict(dict1, category);
    if !dict2.is_null() && pdf_obj_typeof(dict2) == 6i32 {
        if !pdf_lookup_dict(dict2, resname).is_null() {
            return 1i32;
        }
    }
    return 0i32;
}
/* ENABLE_HTML_SVG_OPACITY */
unsafe extern "C" fn spc_html__img_empty(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
) -> libc::c_int {
    let mut src: *mut pdf_obj = 0 as *mut pdf_obj; /* meaning fully opaque */
    let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    let mut id: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    let mut alpha: libc::c_double = 1.0f64;
    /* ENABLE_HTML_SVG_OPACITY */
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut M1: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.a = 1.0f64;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = 1.0f64;
    M.e = (*spe).x_user;
    M.f = (*spe).y_user;
    /* ENABLE_HTML_SVG_TRANSFORM */
    spc_warn(
        spe,
        b"html \"img\" tag found (not completed, plese don\'t use!).\x00" as *const u8
            as *const libc::c_char,
    );
    src = pdf_lookup_dict(attr, b"src\x00" as *const u8 as *const libc::c_char);
    if src.is_null() {
        spc_warn(
            spe,
            b"\"src\" attribute not found for \"img\" tag!\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    transform_info_clear(&mut ti);
    obj = pdf_lookup_dict(attr, b"width\x00" as *const u8 as *const libc::c_char);
    if !obj.is_null() {
        ti.width = atopt(pdf_string_value(obj) as *const libc::c_char);
        ti.flags |= 1i32 << 1i32
    }
    obj = pdf_lookup_dict(attr, b"height\x00" as *const u8 as *const libc::c_char);
    if !obj.is_null() {
        ti.height = atopt(pdf_string_value(obj) as *const libc::c_char);
        ti.flags |= 1i32 << 2i32
    }
    obj = pdf_lookup_dict(attr, b"svg:opacity\x00" as *const u8 as *const libc::c_char);
    if !obj.is_null() {
        alpha = atof(pdf_string_value(obj) as *const libc::c_char);
        if alpha < 0.0f64 || alpha > 1.0f64 {
            spc_warn(
                spe,
                b"Invalid opacity value: %s\x00" as *const u8 as *const libc::c_char,
                pdf_string_value(obj) as *mut libc::c_char,
            );
            alpha = 1.0f64
        }
    }
    /* ENABLE_HTML_SVG_OPCAITY */
    obj = pdf_lookup_dict(
        attr,
        b"svg:transform\x00" as *const u8 as *const libc::c_char,
    );
    if !obj.is_null() {
        let mut p: *const libc::c_char = pdf_string_value(obj) as *const libc::c_char;
        let mut N: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
        while *p as libc::c_int != 0 && error == 0 {
            N.a = 1.0f64;
            N.b = 0.0f64;
            N.c = 0.0f64;
            N.d = 1.0f64;
            N.e = 0.0f64;
            N.f = 0.0f64;
            error = cvt_a_to_tmatrix(&mut N, p, &mut p);
            if error == 0 {
                N.f = -N.f;
                let mut _tmp_a: libc::c_double = 0.;
                let mut _tmp_b: libc::c_double = 0.;
                let mut _tmp_c: libc::c_double = 0.;
                let mut _tmp_d: libc::c_double = 0.;
                _tmp_a = M.a;
                _tmp_b = M.b;
                _tmp_c = M.c;
                _tmp_d = M.d;
                M.a = N.a * _tmp_a + N.b * _tmp_c;
                M.b = N.a * _tmp_b + N.b * _tmp_d;
                M.c = N.c * _tmp_a + N.d * _tmp_c;
                M.d = N.c * _tmp_b + N.d * _tmp_d;
                M.e += N.e * _tmp_a + N.f * _tmp_c;
                M.f += N.e * _tmp_b + N.f * _tmp_d;
                while *p as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                {
                    p = p.offset(1)
                }
                if *p as libc::c_int == ',' as i32 {
                    p = p.offset(1);
                    while *p as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        p = p.offset(1)
                    }
                }
            }
        }
    }
    /* ENABLE_HTML_SVG_TRANSFORM */
    if error != 0 {
        spc_warn(
            spe,
            b"Error in html \"img\" tag attribute.\x00" as *const u8 as *const libc::c_char,
        ); /* Not Tps prefix but... */
        return error;
    } /* op: */
    id = pdf_ximage_findresource(pdf_string_value(src) as *const libc::c_char, options); /* op: */
    if id < 0i32 {
        spc_warn(
            spe,
            b"Could not find/load image: %s\x00" as *const u8 as *const libc::c_char,
            pdf_string_value(src) as *mut libc::c_char,
        ); /* op: gs */
        error = -1i32
    } else {
        let mut res_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        graphics_mode();
        pdf_dev_gsave();
        let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut a: libc::c_int = round(100.0f64 * alpha) as libc::c_int;
        if a != 0i32 {
            res_name = new(
                (strlen(b"_Tps_a100_\x00" as *const u8 as *const libc::c_char)
                    .wrapping_add(1i32 as libc::c_ulong) as uint32_t
                    as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as uint32_t,
            ) as *mut libc::c_char;
            sprintf(
                res_name,
                b"_Tps_a%03d_\x00" as *const u8 as *const libc::c_char,
                a,
            );
            if check_resourcestatus(
                b"ExtGState\x00" as *const u8 as *const libc::c_char,
                res_name,
            ) == 0
            {
                dict = create_xgstate(
                    round(0.01f64 * a as libc::c_double / 0.01f64) * 0.01f64,
                    0i32,
                );
                pdf_doc_add_page_resource(
                    b"ExtGState\x00" as *const u8 as *const libc::c_char,
                    res_name,
                    pdf_ref_obj(dict),
                );
                pdf_release_obj(dict);
            }
            pdf_doc_add_page_content(
                b" /\x00" as *const u8 as *const libc::c_char,
                2i32 as libc::c_uint,
            );
            pdf_doc_add_page_content(res_name, strlen(res_name) as libc::c_uint);
            pdf_doc_add_page_content(
                b" gs\x00" as *const u8 as *const libc::c_char,
                3i32 as libc::c_uint,
            );
            free(res_name as *mut libc::c_void);
        }
        /* ENABLE_HTML_SVG_OPACITY */
        pdf_ximage_scale_image(id, &mut M1, &mut r, &mut ti); /* op: */
        let mut _tmp_a_0: libc::c_double = 0.; /* op: */
        let mut _tmp_b_0: libc::c_double = 0.; /* op: Do */
        let mut _tmp_c_0: libc::c_double = 0.;
        let mut _tmp_d_0: libc::c_double = 0.;
        _tmp_a_0 = M.a;
        _tmp_b_0 = M.b;
        _tmp_c_0 = M.c;
        _tmp_d_0 = M.d;
        M.a = M1.a * _tmp_a_0 + M1.b * _tmp_c_0;
        M.b = M1.a * _tmp_b_0 + M1.b * _tmp_d_0;
        M.c = M1.c * _tmp_a_0 + M1.d * _tmp_c_0;
        M.d = M1.c * _tmp_b_0 + M1.d * _tmp_d_0;
        M.e += M1.e * _tmp_a_0 + M1.f * _tmp_c_0;
        M.f += M1.e * _tmp_b_0 + M1.f * _tmp_d_0;
        pdf_dev_concat(&mut M);
        pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
        res_name = pdf_ximage_get_resname(id);
        pdf_doc_add_page_content(
            b" /\x00" as *const u8 as *const libc::c_char,
            2i32 as libc::c_uint,
        );
        pdf_doc_add_page_content(res_name, strlen(res_name) as libc::c_uint);
        pdf_doc_add_page_content(
            b" Do\x00" as *const u8 as *const libc::c_char,
            3i32 as libc::c_uint,
        );
        pdf_dev_grestore();
        pdf_doc_add_page_resource(
            b"XObject\x00" as *const u8 as *const libc::c_char,
            res_name,
            pdf_ximage_get_reference(id),
        );
        /* ENABLE_HTML_SVG_XXX */
    }
    return error;
}
/* ENABLE_HTML_IMG_SUPPORT */
unsafe extern "C" fn spc_handler_html_default(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_html_ = &mut _html_state; /* treat "open" same as "empty" */
    let mut name: [libc::c_char; 128] = [0; 128]; /* treat "open" same as "empty" */
    let mut attr: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: libc::c_int = 0i32;
    let mut type_0: libc::c_int = 1i32;
    if (*ap).curptr >= (*ap).endptr {
        return 0i32;
    }
    attr = pdf_new_dict();
    error = read_html_tag(
        name.as_mut_ptr(),
        attr,
        &mut type_0,
        &mut (*ap).curptr,
        (*ap).endptr,
    );
    if error != 0 {
        pdf_release_obj(attr);
        return error;
    }
    if streq_ptr(
        name.as_mut_ptr(),
        b"a\x00" as *const u8 as *const libc::c_char,
    ) {
        match type_0 {
            1 => error = spc_html__anchor_open(spe, attr, sd),
            2 => error = spc_html__anchor_close(spe, sd),
            _ => {
                spc_warn(
                    spe,
                    b"Empty html anchor tag???\x00" as *const u8 as *const libc::c_char,
                );
                error = -1i32
            }
        }
    } else if streq_ptr(
        name.as_mut_ptr(),
        b"base\x00" as *const u8 as *const libc::c_char,
    ) {
        if type_0 == 2i32 {
            spc_warn(
                spe,
                b"Close tag for \"base\"???\x00" as *const u8 as *const libc::c_char,
            );
            error = -1i32
        } else {
            error = spc_html__base_empty(spe, attr, sd)
        }
    } else if streq_ptr(
        name.as_mut_ptr(),
        b"img\x00" as *const u8 as *const libc::c_char,
    ) {
        if type_0 == 2i32 {
            spc_warn(
                spe,
                b"Close tag for \"img\"???\x00" as *const u8 as *const libc::c_char,
            );
            error = -1i32
        } else {
            error = spc_html__img_empty(spe, attr)
        }
    }
    pdf_release_obj(attr);
    while (*ap).curptr < (*ap).endptr
        && *(*__ctype_b_loc())
            .offset(*(*ap).curptr.offset(0) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    return error;
}
/* translate wsp* '(' wsp* number (comma-wsp number)? wsp* ')' */
unsafe extern "C" fn cvt_a_to_tmatrix(
    mut M: *mut pdf_tmatrix,
    mut ptr: *const libc::c_char,
    mut nextptr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = ptr;
    let mut n: libc::c_int = 0;
    let mut v: [libc::c_double; 6] = [0.; 6];
    static mut _tkeys: [*const libc::c_char; 7] = [
        b"matrix\x00" as *const u8 as *const libc::c_char,
        b"translate\x00" as *const u8 as *const libc::c_char,
        b"scale\x00" as *const u8 as *const libc::c_char,
        b"rotate\x00" as *const u8 as *const libc::c_char,
        b"skewX\x00" as *const u8 as *const libc::c_char,
        b"skewY\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut k: libc::c_int = 0;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    q = parse_c_ident(&mut p, p.offset(strlen(p) as isize));
    if q.is_null() {
        return -1i32;
    }
    /* parsed transformation key */
    k = 0i32;
    while !_tkeys[k as usize].is_null() && strcmp(q, _tkeys[k as usize]) != 0 {
        k += 1
    }
    free(q as *mut libc::c_void);
    /* handle args */
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    if *p as libc::c_int != '(' as i32 || *p.offset(1) as libc::c_int == 0i32 {
        return -1i32;
    }
    p = p.offset(1);
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    n = 0i32;
    while n < 6i32 && *p as libc::c_int != 0 && *p as libc::c_int != ')' as i32 {
        q = parse_float_decimal(&mut p, p.offset(strlen(p) as isize));
        if q.is_null() {
            break;
        }
        v[n as usize] = atof(q);
        if *p as libc::c_int == ',' as i32 {
            p = p.offset(1)
        }
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
        if *p as libc::c_int == ',' as i32 {
            p = p.offset(1);
            while *p as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            {
                p = p.offset(1)
            }
        }
        free(q as *mut libc::c_void);
        n += 1
    }
    if *p as libc::c_int != ')' as i32 {
        return -1i32;
    }
    p = p.offset(1);
    match k {
        0 => {
            if n != 6i32 {
                return -1i32;
            }
            (*M).a = v[0];
            (*M).c = v[1];
            (*M).b = v[2];
            (*M).d = v[3];
            (*M).e = v[4];
            (*M).f = v[5]
        }
        1 => {
            if n != 1i32 && n != 2i32 {
                return -1i32;
            }
            (*M).d = 1.0f64;
            (*M).a = (*M).d;
            (*M).b = 0.0f64;
            (*M).c = (*M).b;
            (*M).e = v[0];
            (*M).f = if n == 2i32 { v[1] } else { 0.0f64 }
        }
        2 => {
            if n != 1i32 && n != 2i32 {
                return -1i32;
            }
            (*M).a = v[0];
            (*M).d = if n == 2i32 { v[1] } else { v[0] };
            (*M).b = 0.0f64;
            (*M).c = (*M).b;
            (*M).f = 0.0f64;
            (*M).e = (*M).f
        }
        3 => {
            if n != 1i32 && n != 3i32 {
                return -1i32;
            }
            (*M).a = cos(v[0] * 3.14159265358979323846f64 / 180.0f64);
            (*M).c = sin(v[0] * 3.14159265358979323846f64 / 180.0f64);
            (*M).b = -(*M).c;
            (*M).d = (*M).a;
            (*M).e = if n == 3i32 { v[1] } else { 0.0f64 };
            (*M).f = if n == 3i32 { v[2] } else { 0.0f64 }
        }
        4 => {
            if n != 1i32 {
                return -1i32;
            }
            (*M).d = 1.0f64;
            (*M).a = (*M).d;
            (*M).c = 0.0f64;
            (*M).b = tan(v[0] * 3.14159265358979323846f64 / 180.0f64)
        }
        5 => {
            if n != 1i32 {
                return -1i32;
            }
            (*M).d = 1.0f64;
            (*M).a = (*M).d;
            (*M).c = tan(v[0] * 3.14159265358979323846f64 / 180.0f64);
            (*M).b = 0.0f64
        }
        _ => {}
    }
    if !nextptr.is_null() {
        *nextptr = p
    }
    return 0i32;
}
/* ENABLE_HTML_SVG_TRANSFORM */
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_begin_document() -> libc::c_int {
    let mut sd: *mut spc_html_ = &mut _html_state;
    return spc_handler_html__init(sd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_begin_page() -> libc::c_int {
    let mut sd: *mut spc_html_ = &mut _html_state;
    return spc_handler_html__bophook(0 as *mut spc_env, sd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_end_page() -> libc::c_int {
    let mut sd: *mut spc_html_ = &mut _html_state;
    return spc_handler_html__eophook(0 as *mut spc_env, sd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_end_document() -> libc::c_int {
    let mut sd: *mut spc_html_ = &mut _html_state;
    return spc_handler_html__clean(0 as *mut spc_env, sd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_check_special(
    mut buffer: *const libc::c_char,
    mut size: libc::c_int,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    p = buffer;
    endptr = p.offset(size as isize);
    while p < endptr
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    size = endptr.wrapping_offset_from(p) as libc::c_long as libc::c_int;
    if size as libc::c_ulong >= strlen(b"html:\x00" as *const u8 as *const libc::c_char)
        && memcmp(
            p as *const libc::c_void,
            b"html:\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"html:\x00" as *const u8 as *const libc::c_char),
        ) == 0
    {
        return 1i32 != 0;
    }
    return 0i32 != 0;
}
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
pub unsafe extern "C" fn spc_html_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    if !sph.is_null() && !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(b"sph && spe && ap\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-spc_html.c\x00" as *const u8 as
                          *const libc::c_char, 910i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"int spc_html_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    while (*ap).curptr < (*ap).endptr
        && *(*__ctype_b_loc())
            .offset(*(*ap).curptr.offset(0) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    if (*ap)
        .curptr
        .offset(strlen(b"html:\x00" as *const u8 as *const libc::c_char) as isize)
        > (*ap).endptr
        || memcmp(
            (*ap).curptr as *const libc::c_void,
            b"html:\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"html:\x00" as *const u8 as *const libc::c_char),
        ) != 0
    {
        return -1i32;
    }
    (*ap).command = b"\x00" as *const u8 as *const libc::c_char;
    (*sph).key = b"html:\x00" as *const u8 as *const libc::c_char;
    (*sph).exec = Some(
        spc_handler_html_default
            as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
    );
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"html:\x00" as *const u8 as *const libc::c_char) as isize);
    while (*ap).curptr < (*ap).endptr
        && *(*__ctype_b_loc())
            .offset(*(*ap).curptr.offset(0) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    return 0i32;
}