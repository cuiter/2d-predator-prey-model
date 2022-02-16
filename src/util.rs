pub struct Size
{
    pub w: u32,
    pub h: u32
}

impl Size
{
    pub const fn new(w: u32, h: u32) -> Size {
        Size {
            w, h
        }
    }
}