use crate::filter::Kernel;

// Get a default Gaussian 5x5 Kernel
pub fn get_gaussian_kernel() -> Kernel {
    Kernel {
        val: vec![
            0.0037, 0.0147, 0.0256, 0.0147, 0.0037,
            0.0147, 0.0586, 0.0952, 0.0586, 0.0147,
            0.0256, 0.0952, 0.1502, 0.0952, 0.0256,
            0.0147, 0.0586, 0.0952, 0.0586, 0.0147,
            0.0037, 0.0147, 0.0256, 0.0147, 0.0037
        ],
        size: 5,
    }
}

// Get a default outline 3x3 Kernel
pub fn get_outline_kernel() -> Kernel {
    Kernel {
        val: vec![
            -1.0, -1.0, -1.0,
            -1.0, 8.0, -1.0,
            -1.0, -1.0, -1.0,
        ],
        size: 3,
    }
}

// Get a default right sobel 3x3 Kernel
pub fn get_right_sobel_kernel() -> Kernel {
    Kernel {
        val: vec![
            -1.0, 0.0, 1.0,
            -2.0, 0.0, 2.0,
            -1.0, 0.0, 1.0,
        ],
        size: 3,
    }
}

// Get a default bottom sobel 3x3 Kernel
pub fn get_bottom_sobel_kernel() -> Kernel {
    Kernel {
        val: vec![
            -1.0, -2.0, -1.0,
            0.0, 0.0, 0.0,
            1.0, 2.0, 1.0,
        ],
        size: 3,
    }
}

// Get a default sharpening 3x3 Kernel with given strenght
pub fn get_sharpening_kernel(strength: f32) -> Kernel {
    Kernel {
        val: vec![
            0.0, (-1.0/4.0) * strength , 0.0,
            (-1.0/4.0) * strength, ((1.0) * strength) + 1.0f32, (-1.0/4.0) * strength,
            0.0, (-1.0/4.0) * strength, 0.0
        ],
        size: 3,
    }
}

// Get a default sharpening 3x3 Kernel with strength 1.0
#[allow(unused_macros)]
macro_rules! sharpening_kernel {
    ($strength: expr) => {
        filter::get_sharpening_kernel($strength)
    };
    () => {
        filter::get_sharpening_kernel(1.0f32)
    };
}

#[allow(unused_imports)]
pub(crate) use sharpening_kernel;
