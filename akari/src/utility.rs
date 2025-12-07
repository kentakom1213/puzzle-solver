const N1: usize = 1_usize.wrapping_neg();
const UP: (usize, usize) = (N1, 0);
const DOWN: (usize, usize) = (1, 0);
const LEFT: (usize, usize) = (0, N1);
const RIGHT: (usize, usize) = (0, 1);
pub const ADJ: [(usize, usize); 4] = [RIGHT, UP, LEFT, DOWN];

pub trait GridUtility {
    fn adj(&self, h: usize, w: usize) -> impl Iterator<Item = (usize, usize)>;
    fn while_dir(
        &self,
        h: usize,
        w: usize,
        dir: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)>;
}

impl GridUtility for (usize, usize) {
    fn while_dir(
        &self,
        h: usize,
        w: usize,
        dir: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        std::iter::repeat(dir)
            .scan(*self, |cur, (dr, dc)| {
                cur.0 = cur.0.wrapping_add(dr);
                cur.1 = cur.1.wrapping_add(dc);
                Some(*cur)
            })
            .take_while(move |&(nr, nc)| nr < h && nc < w)
    }
    fn adj(&self, h: usize, w: usize) -> impl Iterator<Item = (usize, usize)> {
        ADJ.into_iter()
            .map(|(dr, dc)| (self.0.wrapping_add(dr), self.1.wrapping_add(dc)))
            .filter(move |&(nr, nc)| nr < h && nc < w)
    }
}
