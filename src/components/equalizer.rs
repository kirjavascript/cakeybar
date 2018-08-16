use super::{Component, Bar, gtk, ComponentConfig};
use gtk::prelude::*;
use gtk::{DrawingArea};
use std::thread;
use std::sync::mpsc;
use std::cell::RefCell;
use std::rc::Rc;

use pulse_simple::Record;
use dft;
use dft::{Operation, Plan};

pub struct Equalizer;

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

struct Bars {
    data: Vec<u8>,
}

impl Bars {
    fn new() -> Self {
        Self { data: vec![] }
    }
}

impl Component for Equalizer {
    fn init(container: &gtk::Box, config: &ComponentConfig, bar: &Bar) {

        let (tx, rx) = mpsc::channel();

        let height = 27;

        thread::spawn(move || {

            let p = Record::new("Confectionary", "Record", None, RATE);
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

                // let mut top_freq = 0.0;
                // let mut top_freq_volume = 0.0;
                // for (i, volume) in freqs.iter().enumerate() {
                //     if i > 0 && i < freqs.len() / 2 && volume >= &top_freq_volume {
                //         top_freq = i as f32 * RATE as f32 / freqs.len() as f32;
                //         top_freq_volume = *volume;
                //     }
                // }

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
                        (height as f32 * s.max(0.0) / max) as u8
                    })
                    .collect::<Vec<u8>>();

                tx.send(spectrum).ok();
            }

        });

        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let canvas = DrawingArea::new();
        wrapper.add(&canvas);

        let bars = Rc::new(RefCell::new(Bars::new()));

        canvas.set_size_request(400,height);

        canvas.connect_draw(clone!(bars move |_, cr| {
            cr.set_source_rgb(0.65, 0.26, 1.29);

            for (bar_height, i) in bars.borrow().data.iter().enumerate() {
                if *i != 0 {
                    cr.rectangle(
                        20. * (*i - 1) as f64,
                        height as f64 - bar_height as f64,
                        20.0,
                        bar_height as _,
                        );
                    cr.fill();
                }
            }
            Inhibit(false)
        }));

        gtk::timeout_add(10, clone!((canvas, bars) move || {
            if let Ok(msg) = rx.try_recv() {
                bars.borrow_mut().data = msg.clone();
                canvas.queue_draw();
            }
            gtk::Continue(true)
        }));

        wrapper.show_all();

        Self::init_widget(&wrapper, container, config, bar);
    }
}
