extern crate libc;
extern crate srv;
use self::libc::{c_char, c_int};
use std::ffi::{CString, CStr};
use std::str::from_utf8;
use std::env::args;
use srv::ctypes::*;
use srv::ctypes::ns_msg;
use std::process::exit;
use std::ptr::null;

#[link(name = "resolv")]
extern {
    pub fn __res_query(dname: *const c_char, class: c_int, typef: c_int,
               answer: *const u8, anslen: c_int) -> c_int;
    pub fn ns_initparse(answer: *const u8, len: c_int, dst: *mut ns_msg);
    pub fn ns_parserr(msg: *mut ns_msg, sect: ns_sect_q, which: c_int, rr: *mut ns_rr);
    pub fn ns_sprintrr(msg: *mut ns_msg, rr: *mut ns_rr, b1: *const c_char,
                       b2: *const c_char, buf: *const c_char, buflen: c_int);
}

fn main() {

    let domain = args().nth(1).expect("you must enter a domain name");
    let dns_type = args().nth(2).unwrap_or_else(|| "a".to_string());
    get_ns(domain, dns_type);
}

fn get_ns(domain: String, mut dns_type: String) {
    dns_type = dns_type.to_lowercase();
    let ans_buf = [0u8;4096];
    let mut msg: ns_msg = Default::default();
    let dname = CString::new(&*domain).unwrap();
    unsafe {
        
        let len;
        if dns_type == "ns" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::NS as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "cname" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::CNAME as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "soa" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::SOA as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "ptr" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::PTR as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "mx" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::MX as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "txt" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::TXT as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "aaaa" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::AAAA as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else if dns_type == "srv" {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::SRV as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        } else {
            len = __res_query(dname.as_ptr(), Class::INET as i32, Type::A as i32, &ans_buf as *const u8, ans_buf.len() as i32);
        }

        if len == -1 {
            println!("no such dns record for {}", domain);
            exit(1);
        }
        parser(&ans_buf, len, &mut msg);
    }
}


fn parser(ans_buf: &[u8; 4096], len: c_int, msg: *mut ns_msg) {
    unsafe {
        ns_initparse(ans_buf as *const u8, len, msg);
        let nmsg_answer = (*msg).counts[1] as c_int;

        for i in 0..nmsg_answer {
            let dispbuf = [0i8;4096];
            let mut rr: ns_rr = Default::default();
            ns_parserr(msg, ns_sect_q::ns_s_an, i, &mut rr);
            ns_sprintrr(msg, &mut rr, null::<c_char>(), null::<c_char>(), dispbuf.as_ptr(), dispbuf.len() as i32);

            let c_str = CStr::from_ptr(dispbuf.as_ptr());
            let s = from_utf8(c_str.to_bytes()).unwrap();
            println!("{}", s);
        }
    }

}
