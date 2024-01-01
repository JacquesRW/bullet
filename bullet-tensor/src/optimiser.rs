use bullet_cuda::{util, ops};

use crate::GpuBuffer;

/// A struct intended to hold all network weights and biases
/// needed for training.
pub struct Optimiser {
    size: usize,
    network: GpuBuffer,
    momentum: GpuBuffer,
    velocity: GpuBuffer,
    gradients: GpuBuffer,
}

impl Optimiser {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            network: GpuBuffer::new(size),
            momentum: GpuBuffer::new(size),
            velocity: GpuBuffer::new(size),
            gradients: GpuBuffer::new(size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn zero_gradient(&self) {
        util::set_zero(self.gradients.ptr(), self.gradients.size());
    }

    /// Pointer to network buffer starting at `network.ptr() + index`.
    pub fn weights_offset(&self, index: usize) -> *mut f32 {
        assert!(
            index < self.size,
            "Index out of bounds: {index} >= {}!",
            self.size
        );
        unsafe { self.network.ptr().add(index) }
    }

    /// Pointer to gradient buffer starting at `gradient.ptr() + index`.
    pub fn gradients_offset(&self, index: usize) -> *mut f32 {
        assert!(
            index < self.size,
            "Index out of bounds: {index} >= {}!",
            self.size
        );
        unsafe { self.gradients.ptr().add(index) }
    }

    pub fn update(&self, decay: f32, adj: f32, rate: f32) {
        let decay_gamma = 1.0 - decay * rate;
        unsafe {
            ops::update_weights(
                self.size,
                decay_gamma,
                adj,
                rate,
                self.network.ptr(),
                self.momentum.ptr(),
                self.velocity.ptr(),
                self.gradients.ptr(),
            );
        }
    }

    pub fn load_weights_from_cpu(&self, network: &[f32]) {
        self.network.load_from_cpu(network);
    }

    pub fn load_from_cpu(&self, network: &[f32], momentum: &[f32], velocity: &[f32]) {
        self.network.load_from_cpu(network);
        self.momentum.load_from_cpu(momentum);
        self.velocity.load_from_cpu(velocity);
    }

    pub fn write_weights_to_buffer(&self, buf: &mut [f32]) {
        self.network.write_to_cpu(buf);
    }

    pub fn write_to_cpu(&self, network: &mut [f32], momentum: &mut [f32], velocity: &mut [f32]) {
        self.network.write_to_cpu(network);
        self.momentum.write_to_cpu(momentum);
        self.velocity.write_to_cpu(velocity);
    }
}
