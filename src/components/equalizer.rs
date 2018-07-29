use libpulse_sys;
use libpulse_simple_sys;

use std::os::raw::c_void;
use std::{thread, ptr, mem};
use libc::{c_char, c_int, int16_t};
use libpulse_sys::sample::{pa_sample_spec, pa_sample_format_t};
use libpulse_sys::def::pa_buffer_attr;
use libpulse_sys::channelmap::pa_channel_map;

pub fn test() {
    thread::spawn(move || {
        unsafe {
            let c_char_null: *const c_char = ptr::null();
            let name = b"cake\0".as_ptr() as _;
            let dir = libpulse_sys::stream::pa_stream_direction_t::Record;
            let stream_name = b"music\0".as_ptr() as _;
            let spec = pa_sample_spec {
                format: pa_sample_format_t::S16le,
                rate: 44100,
                channels: 2,
            };
            let ss: *const pa_sample_spec = &spec as _;
            let map_null: *const pa_channel_map = ptr::null();
            let attr_null: *const pa_buffer_attr = ptr::null();
            let error: *mut i32 = 0 as _;

            let s = libpulse_simple_sys::pa_simple_new(
                c_char_null, // server: *const c_char,
                name, // name: *const c_char,
                dir, // dir: pa_stream_direction_t,
                c_char_null, // dev: *const c_char,
                stream_name, // stream_name: *const c_char,
                ss, // ss: *const pa_sample_spec,
                map_null, // map: *const pa_channel_map,
                attr_null, // attr: *const pa_buffer_attr,
                error,// error: *mut i32
            );

            let mut n = 0;
            const LEN: usize = 512;
            const BUFFER_LEN: usize = 2048;

            let mut audio_out_r: [int16_t; BUFFER_LEN] = [0; BUFFER_LEN];
            let mut audio_out_l: [int16_t; BUFFER_LEN] = [0; BUFFER_LEN];

            loop {
                let mut buf: [int16_t; LEN] = [0; LEN];
                let buf_ptr: *mut c_void = buf.as_ptr() as _;

                // TODO: audio-> terminate

                libpulse_simple_sys::pa_simple_read(
                    s, // s: *mut pa_simple,
                    buf_ptr,// data: *mut c_void,
                    mem::size_of_val(&buf),// bytes: usize,
                    error,// error: *mut i32
                );

                let mut i = 0;

                while i < LEN {
                    if n > BUFFER_LEN {
                        panic!("tried to write past the audio buffer");
                    }
                    audio_out_l[n] = buf[i];
                    audio_out_r[n] = buf[i + 1];

                    n += 1;
                    if n == 2048 - 1 {
                        n = 0;
                    }
                    i += 2;
                }

                info!("{:?}", &audio_out_l[..8])
            }
        }
    });
}
