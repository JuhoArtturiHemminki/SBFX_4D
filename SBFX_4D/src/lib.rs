use std::time::Instant;
use std::sync::OnceLock;

fn read_system_clock() -> u64 {
    static START_TIME: OnceLock<Instant> = OnceLock::new();
    START_TIME.get_or_init(Instant::now).elapsed().as_nanos() as u64
}

#[cfg(target_arch = "aarch64")]
fn sbfx_projection_hardware(delta: u64) -> i32 {
    let result: i32;
    unsafe {
        std::arch::asm!(
            "sbfx w0, {delta:w}, #0, #1",
            "mov w1, #9900",
            "and w0, w0, w1",
            "mov w2, #-4950",
            "add {result:w}, w2, w0",
            delta = in(reg) delta,
            result = out(reg) result,
            out("w0") _, out("w1") _, out("w2") _,
        );
    }
    result
}

#[cfg(target_arch = "x86_64")]
fn sbfx_projection_hardware(delta: u64) -> i32 {
    let result: i32;
    unsafe {
        std::arch::asm!(
            "mov rax, {delta}",
            "and rax, 1",
            "neg rax",
            "and rax, 9900",
            "sub rax, 4950",
            "mov {result:e}, eax",
            delta = in(reg) delta,
            result = out(reg) result,
            out("rax") _,
        );
    }
    result
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn sbfx_projection_hardware(delta: u64) -> i32 {
    if (delta & 1) == 1 { 4950 } else { -4950 }
}

pub fn sbfx_projection_core() -> i32 {
    let current_time = read_system_clock();
    sbfx_projection_hardware(current_time)
}

pub struct WaveDemuxer {
    frequency_base: [f64; 8],
}

impl WaveDemuxer {
    pub fn new() -> Self {
        let mut base = [0.0; 8];
        for i in 0..8 {
            base[i] = (i as f64) * 1.618033988749895;
        }
        WaveDemuxer { frequency_base: base }
    }

    pub fn demux_signal(&self, photon_mass: f64) -> [u8; 8] {
        let mut output = [0; 8];
        for i in 0..8 {
            if self.frequency_base[i] > 0.0 {
                let projection = photon_mass / self.frequency_base[i];
                let wave_state = (projection * std::f64::consts::PI).sin();
                if wave_state.abs() < 0.00001 {
                    output[i] = 1;
                }
            }
        }
        output
    }
}
