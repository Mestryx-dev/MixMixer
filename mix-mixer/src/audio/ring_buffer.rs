use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Single-producer single-consumer ring buffer for interleaved stereo f32 samples.
pub struct SpscRingBuffer {
    data: UnsafeCell<Box<[f32]>>,
    capacity: usize,
    write: AtomicUsize,
    read: AtomicUsize,
}

unsafe impl Sync for SpscRingBuffer {}

impl SpscRingBuffer {
    pub fn new(capacity_samples: usize) -> Self {
        Self {
            data: UnsafeCell::new(vec![0.0; capacity_samples].into_boxed_slice()),
            capacity: capacity_samples,
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
        }
    }

    /// Push samples; returns number of samples actually written (may truncate if full).
    pub fn push(&self, samples: &[f32]) -> usize {
        let mut written = 0usize;
        for &sample in samples {
            let w = self.write.load(Ordering::Acquire);
            let r = self.read.load(Ordering::Acquire);
            let used = w.wrapping_sub(r);
            if used >= self.capacity {
                break;
            }
            unsafe {
                let slot = &mut *self.data.get();
                slot[w % self.capacity] = sample;
            }
            self.write.store(w.wrapping_add(1), Ordering::Release);
            written += 1;
        }
        written
    }

    /// Pop up to `out.len()` samples; pads with silence if underflow.
    pub fn pop_into(&self, out: &mut [f32]) -> usize {
        let mut read_count = 0usize;
        for sample in out.iter_mut() {
            let w = self.write.load(Ordering::Acquire);
            let r = self.read.load(Ordering::Acquire);
            if r == w {
                *sample = 0.0;
            } else {
                unsafe {
                    let slot = &*self.data.get();
                    *sample = slot[r % self.capacity];
                }
                self.read.store(r.wrapping_add(1), Ordering::Release);
                read_count += 1;
            }
        }
        read_count
    }

    pub fn available(&self) -> usize {
        let w = self.write.load(Ordering::Acquire);
        let r = self.read.load(Ordering::Acquire);
        w.wrapping_sub(r).min(self.capacity)
    }

    pub fn clear(&self) {
        self.read
            .store(self.write.load(Ordering::Acquire), Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_roundtrip() {
        let rb = SpscRingBuffer::new(8);
        assert_eq!(rb.push(&[1.0, 2.0, 3.0]), 3);
        let mut out = [0.0; 3];
        assert_eq!(rb.pop_into(&mut out), 3);
        assert_eq!(out, [1.0, 2.0, 3.0]);
    }
}
