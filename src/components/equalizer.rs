use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{Label, StyleContextExt};

use dft;
use pulse_simple::Record;
use dft::{Operation, Plan};
use std::thread;

use std::sync::mpsc;

pub struct Equalizer { }

fn analyze_channel(plan: &Plan<f64>, data: &[[f32; 2]], channel: usize) -> Vec<f32> {
    let mut input = Vec::with_capacity(data.len());
    for x in data {
        input.push(x[channel] as f64);
    }

    dft::transform(&mut input, plan);
    let output = dft::unpack(&input);

    let mut result = Vec::with_capacity(data.len());
    for ref c in output {
        result.push(c.norm() as f32);
    }
    result
}

const RATE: u32 = 48000;
const WINDOW: usize = 2048;
const FREQS_PER_COLUMN: usize = 20;

impl Component for Equalizer {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {

            let p = Record::new("Example", "Record", None, RATE);
            let mut plan: Plan<f64> = Plan::new(Operation::Forward, WINDOW);

            // Fill:
            let mut data = Vec::with_capacity(WINDOW);
            for _ in 0..WINDOW {
                data.push([0.0, 0.0]);
            }

            // Record:
            let mut max: f32 = 0.0;
            loop {
                p.read(&mut data[..]);
                let freqs = analyze_channel(&mut plan, &data[..], 0);

                let mut top_freq = 0.0;
                let mut top_freq_volume = 0.0;
                for (i, volume) in freqs.iter().enumerate() {
                    if i > 0 && i < freqs.len() / 2 && volume >= &top_freq_volume {
                        top_freq = i as f32 * RATE as f32 / freqs.len() as f32;
                        top_freq_volume = *volume;
                    }
                }

                let mut spectrum = Vec::with_capacity(WINDOW / FREQS_PER_COLUMN);
                max *= 0.95;  // Dampen
                for column in 0..(WINDOW / FREQS_PER_COLUMN) {
                    let c1 = column * FREQS_PER_COLUMN;
                    let c2 = (column + 1) * FREQS_PER_COLUMN;
                    let mut sum: f32 = 0.0;
                    for x in c1..c2 {
                        sum += freqs[x];
                    }
                    if column > 0 && column < WINDOW / FREQS_PER_COLUMN / 2 {
                        spectrum.push(sum);
                        max = max.max(sum);
                    }
                }
                // println!("top: {} Hz at volume {}", top_freq, top_freq_volume);

//                 print!("[");
//                 for s in spectrum.clone() {
//                     print!("{}, ", (9.0 * s.max(0.0) / max) as u8);
//                 }
//                 println!("]");
//
                let spectrum = spectrum
                    .iter()
                    .map(|s| {
                        (27.0 * s.max(0.0) / max) as u8
                    })
                    .collect::<Vec<u8>>();

                tx.send(spectrum).ok();
            }

        });

        // let label = Label::new(None);
        // label.set_text(&"test");
        // label.show();

        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);



        gtk::timeout_add(10, clone!(wrapper move || {
            if let Ok(mut msg) = rx.try_recv() {
                // info!("{:?}", msg);
                for child in wrapper.get_children() {
                    child.destroy();
                }

                for s in msg {
                    let label = Label::new(None);
                    wrapper.add(&label);
                    // label.set_text(&format!("{}", s));
                    label.set_size_request(15, 0);
                    label.set_margin_bottom(s as i32);
                    if let Some(ctx) = label.get_style_context() {
                        ctx.add_class("bar");
                    }
                    label.show();
                }

                // wrapper.show_all();
            }
            gtk::Continue(true)
        }));

        wrapper.show();

        Self::init_widget(&wrapper, container, config, bar);
    }
}

//
// use libpulse_sys;
// use libpulse_simple_sys;

// use std::os::raw::c_void;
// use std::{thread, ptr, mem};
// use libc::{c_char, int16_t};
// use libpulse_sys::sample::{pa_sample_spec, pa_sample_format_t};
// use libpulse_sys::def::pa_buffer_attr;
// use libpulse_sys::channelmap::pa_channel_map;

// mod rusty_bars;
// use rusty_bars::pulse::PulseAudioMainloop;
// use rusty_bars::viz_runner::VizRunner;

// pub fn test() {
//     thread::spawn(move || {
//         let mainloop = PulseAudioMainloop::new();
//         VizRunner::new(&mainloop);
//         mainloop.run();
//     });
// }

// pub fn test_1() {
//     thread::spawn(move || {
//         unsafe {
//             let c_char_null: *const c_char = ptr::null();
//             let name = b"cake\0".as_ptr() as _;
//             let dir = libpulse_sys::stream::pa_stream_direction_t::Record;
//             let stream_name = b"music\0".as_ptr() as _;
//             let spec = pa_sample_spec {
//                 format: pa_sample_format_t::S16le,
//                 rate: 44100,
//                 channels: 2,
//             };
//             let ss: *const pa_sample_spec = &spec as _;
//             let map_null: *const pa_channel_map = ptr::null();
//             let attr_null: *const pa_buffer_attr = ptr::null();
//             let error: *mut i32 = 0 as _;

//             let s = libpulse_simple_sys::pa_simple_new(
//                 c_char_null, // server: *const c_char,
//                 name, // name: *const c_char,
//                 dir, // dir: pa_stream_direction_t,
//                 c_char_null, // dev: *const c_char,
//                 stream_name, // stream_name: *const c_char,
//                 ss, // ss: *const pa_sample_spec,
//                 map_null, // map: *const pa_channel_map,
//                 attr_null, // attr: *const pa_buffer_attr,
//                 error,// error: *mut i32
//                 );

//             let mut n = 0;
//             const LEN: usize = 512;
//             const BUFFER_LEN: usize = 2048;

//             loop {
//                 let mut buf: [int16_t; LEN] = [0; LEN];
//                 let buf_ptr: *mut c_void = buf.as_ptr() as _;

//                 // TODO: audio-> terminate

//                 libpulse_simple_sys::pa_simple_read(
//                     s, // s: *mut pa_simple,
//                     buf_ptr,// data: *mut c_void,
//                     mem::size_of_val(&buf),// bytes: usize,
//                     error,// error: *mut i32
//                     );

//                 let mut audio_out_r: [int16_t; BUFFER_LEN] = [0; BUFFER_LEN];
//                 let mut audio_out_l: [int16_t; BUFFER_LEN] = [0; BUFFER_LEN];

//                 // ???
//                 let mut i = 0;
//                 while i < LEN {
//                     if n > BUFFER_LEN {
//                         panic!("tried to write past the audio buffer");
//                     }
//                     audio_out_l[n] = buf[i];
//                     audio_out_r[n] = buf[i + 1];

//                     n += 1;
//                     if n == 2048 - 1 {
//                         n = 0;
//                     }
//                     i += 2;
//                 }

//                 // ???
//                 const unknown: usize = 2 * (BUFFER_LEN / 2 + 1);

//                 let mut inl: [int16_t; unknown] = [0; unknown];
//                 let mut inr: [int16_t; unknown] = [0; unknown];

//                 let mut silence = true;
//                 for i in 0..unknown {
//                     if i < BUFFER_LEN {
//                         inl[i] = audio_out_l[i];
//                         inr[i] = audio_out_r[i];
//                         if inl[i] > 0 || inr[i] > 0 {
//                             silence = false;
//                         }
//                     } else {
//                         inl[i] = 0;
//                         inr[i] = 0;
//                     }
//                 }


//                 info!("{:?}", &inr[..2]);
//                 // info!("{:?}", silence);
//             }
//         }
//     });
// }
