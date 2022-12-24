use cairo::*;

pub struct Display {
    disp: ImageSurface,
}

impl Display {
    pub fn new() -> Self {
        Self { disp: ImageSurface::create(Format::A8, 64, 32).  }
    }
}
