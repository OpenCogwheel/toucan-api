use drm::buffer::DrmFourcc;

#[derive(Debug, Clone)]
pub struct Vec2<T> {
    pub x: T, pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub enum ColorFormat {
    Abgr1555,
    Abgr16161616f,
    Abgr2101010,
    Abgr4444,
    Abgr8888,
    Argb1555,
    Argb16161616f,
    Argb2101010,
    Argb4444,
    Argb8888,
    Axbxgxrx106106106106,
    Ayuv,
    Bgr233,
    Bgr565,
    Bgr565_a8,
    Bgr888,
    Bgr888_a8,
    Bgra1010102,
    Bgra4444,
    Bgra5551,
    Bgra8888,
    Bgrx1010102,
    Bgrx4444,
    Bgrx5551,
    Bgrx8888,
    Bgrx8888_a8,
    Big_endian,
    C8,
    Gr1616,
    Gr88,
    Nv12,
    Nv15,
    Nv16,
    Nv21,
    Nv24,
    Nv42,
    Nv61,
    P010,
    P012,
    P016,
    P210,
    Q401,
    Q410 ,
    R16,
    R8,
    Rg1616,
    Rg88,
    Rgb332,
    Rgb565,
    Rgb565_a8,
    Rgb888,
    Rgb888_a8,
    Rgba1010102,
    Rgba4444,
    Rgba5551,
    Rgba8888,
    Rgbx1010102,
    Rgbx4444,
    Rgbx5551,
    Rgbx8888,
    Rgbx8888_a8,
    Uyvy,
    Vuy101010,
    Vuy888,
    Vyuy,
    X0l0,
    X0l2,
    Xbgr1555,
    Xbgr16161616f,
    Xbgr2101010,
    Xbgr4444,
    Xbgr8888,
    Xbgr8888_a8,
    Xrgb1555,
    Xrgb16161616f,
    Xrgb2101010,
    Xrgb4444,
    Xrgb8888,
    Xrgb8888_a8,
    Xvyu12_16161616,
    Xvyu16161616,
    Xvyu2101010,
    Xyuv8888,
    Y0l0,
    Y0l2,
    Y210,
    Y212,
    Y216,
    Y410,
    Y412,
    Y416,
    Yuv410,
    Yuv411,
    Yuv420,
    Yuv420_10bit,
    Yuv420_8bit,
    Yuv422,
    Yuv444,
    Yuyv,
    Yvu410,
    Yvu411,
    Yvu420,
    Yvu422,
    Yvu444,
    Yvyu,
}

impl ColorFormat {
    pub fn to_drm(&self) -> DrmFourcc {
        match self {
            Self::Abgr1555 => return DrmFourcc::Abgr1555,
            Self::Abgr16161616f => return DrmFourcc::Abgr16161616f,
            Self::Abgr2101010 => return DrmFourcc::Abgr2101010,
            Self::Abgr4444 => return DrmFourcc::Abgr4444,
            Self::Abgr8888 => return DrmFourcc::Abgr8888,
            Self::Argb1555 => return DrmFourcc::Argb1555,
            Self::Argb16161616f => return DrmFourcc::Argb16161616f,
            Self::Argb2101010 => return DrmFourcc::Argb2101010,
            Self::Argb4444 => return DrmFourcc::Argb4444,
            Self::Argb8888 => return DrmFourcc::Argb8888,
            Self::Axbxgxrx106106106106 => return DrmFourcc::Axbxgxrx106106106106,
            Self::Ayuv => return DrmFourcc::Ayuv,
            Self::Bgr233 => return DrmFourcc::Bgr233,
            Self::Bgr565 => return DrmFourcc::Bgr565,
            Self::Bgr565_a8 => return DrmFourcc::Bgr565_a8,
            Self::Bgr888 => return DrmFourcc::Bgr888,
            Self::Bgr888_a8 => return DrmFourcc::Bgr888_a8,
            Self::Bgra1010102 => return DrmFourcc::Bgra1010102,
            Self::Bgra4444 => return DrmFourcc::Bgra4444,
            Self::Bgra5551 => return DrmFourcc::Bgra5551,
            Self::Bgra8888 => return DrmFourcc::Bgra8888,
            Self::Bgrx1010102 => return DrmFourcc::Bgrx1010102,
            Self::Bgrx4444 => return DrmFourcc::Bgrx4444,
            Self::Bgrx5551 => return DrmFourcc::Bgrx5551,
            Self::Bgrx8888 => return DrmFourcc::Bgrx8888,
            Self::Bgrx8888_a8 => return DrmFourcc::Bgrx8888_a8,
            Self::Big_endian => return DrmFourcc::Big_endian,
            Self::C8 => return DrmFourcc::C8,
            Self::Gr1616 => return DrmFourcc::Gr1616,
            Self::Gr88 => return DrmFourcc::Gr88,
            Self::Nv12 => return DrmFourcc::Nv12,
            Self::Nv15 => return DrmFourcc::Nv15,
            Self::Nv16 => return DrmFourcc::Nv16,
            Self::Nv21 => return DrmFourcc::Nv21,
            Self::Nv24 => return DrmFourcc::Nv24,
            Self::Nv42 => return DrmFourcc::Nv42,
            Self::Nv61 => return DrmFourcc::Nv61,
            Self::P010 => return DrmFourcc::P010,
            Self::P012 => return DrmFourcc::P012,
            Self::P016 => return DrmFourcc::P016,
            Self::P210 => return DrmFourcc::P210,
            Self::Q401 => return DrmFourcc::Q401,
            Self::Q410  => return DrmFourcc::Q410 ,
            Self::R16 => return DrmFourcc::R16,
            Self::R8 => return DrmFourcc::R8,
            Self::Rg1616 => return DrmFourcc::Rg1616,
            Self::Rg88 => return DrmFourcc::Rg88,
            Self::Rgb332 => return DrmFourcc::Rgb332,
            Self::Rgb565 => return DrmFourcc::Rgb565,
            Self::Rgb565_a8 => return DrmFourcc::Rgb565_a8,
            Self::Rgb888 => return DrmFourcc::Rgb888,
            Self::Rgb888_a8 => return DrmFourcc::Rgb888_a8,
            Self::Rgba1010102 => return DrmFourcc::Rgba1010102,
            Self::Rgba4444 => return DrmFourcc::Rgba4444,
            Self::Rgba5551 => return DrmFourcc::Rgba5551,
            Self::Rgba8888 => return DrmFourcc::Rgba8888,
            Self::Rgbx1010102 => return DrmFourcc::Rgbx1010102,
            Self::Rgbx4444 => return DrmFourcc::Rgbx4444,
            Self::Rgbx5551 => return DrmFourcc::Rgbx5551,
            Self::Rgbx8888 => return DrmFourcc::Rgbx8888,
            Self::Rgbx8888_a8 => return DrmFourcc::Rgbx8888_a8,
            Self::Uyvy => return DrmFourcc::Uyvy,
            Self::Vuy101010 => return DrmFourcc::Vuy101010,
            Self::Vuy888 => return DrmFourcc::Vuy888,
            Self::Vyuy => return DrmFourcc::Vyuy,
            Self::X0l0 => return DrmFourcc::X0l0,
            Self::X0l2 => return DrmFourcc::X0l2,
            Self::Xbgr1555 => return DrmFourcc::Xbgr1555,
            Self::Xbgr16161616f => return DrmFourcc::Xbgr16161616f,
            Self::Xbgr2101010 => return DrmFourcc::Xbgr2101010,
            Self::Xbgr4444 => return DrmFourcc::Xbgr4444,
            Self::Xbgr8888 => return DrmFourcc::Xbgr8888,
            Self::Xbgr8888_a8 => return DrmFourcc::Xbgr8888_a8,
            Self::Xrgb1555 => return DrmFourcc::Xrgb1555,
            Self::Xrgb16161616f => return DrmFourcc::Xrgb16161616f,
            Self::Xrgb2101010 => return DrmFourcc::Xrgb2101010,
            Self::Xrgb4444 => return DrmFourcc::Xrgb4444,
            Self::Xrgb8888 => return DrmFourcc::Xrgb8888,
            Self::Xrgb8888_a8 => return DrmFourcc::Xrgb8888_a8,
            Self::Xvyu12_16161616 => return DrmFourcc::Xvyu12_16161616,
            Self::Xvyu16161616 => return DrmFourcc::Xvyu16161616,
            Self::Xvyu2101010 => return DrmFourcc::Xvyu2101010,
            Self::Xyuv8888 => return DrmFourcc::Xyuv8888,
            Self::Y0l0 => return DrmFourcc::Y0l0,
            Self::Y0l2 => return DrmFourcc::Y0l2,
            Self::Y210 => return DrmFourcc::Y210,
            Self::Y212 => return DrmFourcc::Y212,
            Self::Y216 => return DrmFourcc::Y216,
            Self::Y410 => return DrmFourcc::Y410,
            Self::Y412 => return DrmFourcc::Y412,
            Self::Y416 => return DrmFourcc::Y416,
            Self::Yuv410 => return DrmFourcc::Yuv410,
            Self::Yuv411 => return DrmFourcc::Yuv411,
            Self::Yuv420 => return DrmFourcc::Yuv420,
            Self::Yuv420_10bit => return DrmFourcc::Yuv420_10bit,
            Self::Yuv420_8bit => return DrmFourcc::Yuv420_8bit,
            Self::Yuv422 => return DrmFourcc::Yuv422,
            Self::Yuv444 => return DrmFourcc::Yuv444,
            Self::Yuyv => return DrmFourcc::Yuyv,
            Self::Yvu410 => return DrmFourcc::Yvu410,
            Self::Yvu411 => return DrmFourcc::Yvu411,
            Self::Yvu420 => return DrmFourcc::Yvu420,
            Self::Yvu422 => return DrmFourcc::Yvu422,
            Self::Yvu444 => return DrmFourcc::Yvu444,
            Self::Yvyu => return DrmFourcc::Yvyu,
        }
    }
}