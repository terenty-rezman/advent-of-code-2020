struct Map<'a> {
    rows: Vec<&'a [u8]>,
    w: usize,
    h: usize,
}

impl<'a> Map<'a> {
    fn new(text: &str) -> Map {
        let rows: Vec<_> = text.lines().map(|l| l.as_bytes()).collect();
        let w = rows[0].len();
        let h = rows.len();

        Map { rows, w, h }
    }

    fn index(&self, i: usize, j: usize) -> Option<u8> {
        if j > self.h - 1 {
            None
        } else {
            Some(self.rows[j][i % self.w])
        }
    }
}

struct SlopeIterator<'a> {
    map: &'a Map<'a>,
    i: usize,
    j: usize,
    i_step: usize,
    j_step: usize,
}

impl<'a> SlopeIterator<'a> {
    fn new(map: &'a Map, i_step: usize, j_step: usize) -> Self {
        SlopeIterator {
            map,
            i: 0,
            j: 0,
            i_step,
            j_step,
        }
    }
}

impl<'a> Iterator for SlopeIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.i += self.i_step;
        self.j += self.j_step;
        self.map.index(self.i, self.j)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let map = Map::new(input);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mul = slopes
        .map(|(i_step, j_step)| {
            SlopeIterator::new(&map, i_step, j_step)
                .filter(|&b| b == b'#')
                .count()
        })
        .into_iter()
        .product::<usize>();

    println!("{}", &mul);
}
