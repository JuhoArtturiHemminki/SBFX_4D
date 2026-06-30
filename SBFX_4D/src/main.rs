use sbfx_4d::{sbfx_projection_core, WaveDemuxer};
use std::time::Instant;

fn main() {
    let demuxer = WaveDemuxer::new();
    let sample_photon_mass = 42.069;
    
    let start = Instant::now();
    let mut hardware_accumulation: i32 = 0;
    let mut signal_matrix = [0u8; 8];

    for _ in 0..1_000_000 {
        hardware_accumulation = hardware_accumulation.wrapping_add(sbfx_projection_core());
        signal_matrix = demuxer.demux_signal(sample_photon_mass);
    }
    
    let duration = start.elapsed();
    
    println!("{:?}", duration);
    println!("{}", hardware_accumulation);
    println!("{:?}", signal_matrix);
}
