# Game Of Life state parse

Example:

```rust
use gol_builder::GoLBuilder

/// Your state
#[derive(Debug, PartialEq, Eq, Clone)]
struct GoL {
    rows: usize,
    cols: usize,
    lives: HashSet<(usize, usize)>,
}

/// Implement GoLBuilder trait
impl GoLBuilder for GoL {
    fn empty(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            lives: Default::default(),
        }
    }
    fn live(mut self, row: usize, col: usize) -> Self {
        self.lives.insert((row, col));
        self
    }
    fn dead(mut self, row: usize, col: usize) -> Self {
        self.lives.remove(&(row, col));
        self
    }
}

#[test]
fn should_create_no_trivial_state() {
    // Consider to use unindent crate
    let gol: GoL = "..*.\n\
        .+..\n\
        ....\n\
        ..**\n\
        ++.."
        .build_gol()
        .unwrap();

    let expected = GoL::empty(5, 4)
        .live(0, 2)
        .live(1, 1)
        .live(3, 2)
        .live(3, 3)
        .live(4, 0)
        .live(4, 1);

    assert_eq!(expected, gol);
}

```