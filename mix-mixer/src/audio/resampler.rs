use rubato::{FftFixedIn, Resampler};

use crate::error::{Error, Result};

/// Resample mono or interleaved stereo f32 to target rate.
pub struct StereoResampler {
    left: FftFixedIn<f32>,
    right: FftFixedIn<f32>,
    input_rate: u32,
    output_rate: u32,
    channels: usize,
}

impl StereoResampler {
    pub fn new(input_rate: u32, output_rate: u32, chunk_frames: usize) -> Result<Self> {
        if input_rate == output_rate {
            return Err(Error::audio("resampler not needed for identical rates"));
        }

        let left = FftFixedIn::<f32>::new(input_rate as usize, output_rate as usize, chunk_frames, 1, 1)
            .map_err(|e| Error::audio(format!("resampler init left: {e}")))?;
        let right = FftFixedIn::<f32>::new(input_rate as usize, output_rate as usize, chunk_frames, 1, 1)
            .map_err(|e| Error::audio(format!("resampler init right: {e}")))?;

        Ok(Self {
            left,
            right,
            input_rate,
            output_rate,
            channels: 2,
        })
    }

    pub fn process_interleaved(&mut self, input: &[f32]) -> Result<Vec<f32>> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let frames = input.len() / self.channels;
        let mut left_in = vec![vec![0.0f32; frames]];
        let mut right_in = vec![vec![0.0f32; frames]];

        for (i, chunk) in input.chunks(self.channels).enumerate() {
            left_in[0][i] = chunk.first().copied().unwrap_or(0.0);
            right_in[0][i] = chunk.get(1).copied().unwrap_or(chunk[0]);
        }

        let left_out = self
            .left
            .process(&left_in, None)
            .map_err(|e| Error::audio(format!("resample left: {e}")))?;
        let right_out = self
            .right
            .process(&right_in, None)
            .map_err(|e| Error::audio(format!("resample right: {e}")))?;

        let out_frames = left_out[0].len().min(right_out[0].len());
        let mut interleaved = Vec::with_capacity(out_frames * 2);
        for i in 0..out_frames {
            interleaved.push(left_out[0][i]);
            interleaved.push(right_out[0][i]);
        }

        Ok(interleaved)
    }

    pub fn input_rate(&self) -> u32 {
        self.input_rate
    }

    pub fn output_rate(&self) -> u32 {
        self.output_rate
    }
}

/// Convert arbitrary channel count to stereo interleaved f32.
pub fn to_stereo_interleaved(samples: &[f32], channels: u16) -> Vec<f32> {
    match channels {
        1 => samples.iter().flat_map(|&s| [s, s]).collect(),
        2 => samples.to_vec(),
        n => {
            let ch = n as usize;
            let frames = samples.len() / ch;
            let mut out = Vec::with_capacity(frames * 2);
            for frame in samples.chunks(ch) {
                let l = frame.first().copied().unwrap_or(0.0);
                let r = frame.get(1).copied().unwrap_or(l);
                out.push(l);
                out.push(r);
            }
            out
        }
    }
}

/// Convert stereo interleaved to target channel count for output device.
pub fn from_stereo_interleaved(stereo: &[f32], channels: u16) -> Vec<f32> {
    match channels {
        1 => stereo
            .chunks(2)
            .map(|c| (c[0] + c.get(1).copied().unwrap_or(c[0])) * 0.5)
            .collect(),
        2 => stereo.to_vec(),
        n => {
            let ch = n as usize;
            let mut out = Vec::with_capacity(stereo.len() / 2 * ch);
            for frame in stereo.chunks(2) {
                let l = frame[0];
                let r = frame.get(1).copied().unwrap_or(l);
                for i in 0..ch {
                    out.push(if i == 0 { l } else if i == 1 { r } else { 0.0 });
                }
            }
            out
        }
    }
}
