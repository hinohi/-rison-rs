pub trait Read {
    fn next(&mut self) -> Option<u8>;

    fn peak(&mut self) -> Option<u8>;

    fn position(&self) -> usize;

    fn eat_char(&mut self);
}

pub struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead { slice, index: 0 }
    }
}

impl<'a> Read for SliceRead<'a> {
    fn next(&mut self) -> Option<u8> {
        if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn peak(&mut self) -> Option<u8> {
        if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        }
    }

    fn position(&self) -> usize {
        self.index
    }

    fn eat_char(&mut self) {
        self.index += 1;
    }
}
