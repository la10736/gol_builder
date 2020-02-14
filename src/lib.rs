pub trait GoLBuilder {
    fn empty(rows: usize, cols: usize) -> Self;
    fn live(self, row: usize, col: usize) -> Self;
    fn dead(self, row: usize, col: usize) -> Self;
}

pub trait BuildGol<G> {
    type Err;

    fn build_gol(self) -> Result<G, Self::Err>;
}

impl<S: AsRef<str>, G: GoLBuilder> BuildGol<G> for S {
    type Err = String;

    fn build_gol(self: S) -> Result<G, Self::Err> {
        let rows = self.as_ref().lines().count();
        let cols = self.as_ref().lines().map(|l| l.len()).max().unwrap_or(0);
        Ok(self
            .as_ref()
            .lines()
            .enumerate()
            .fold(G::empty(rows, cols), |state, (row, l)| {
                l.chars()
                    .enumerate()
                    .fold(state, move |state, (col, c)| match c {
                        '.' | ' ' => state.dead(row, col),
                        _ => state.live(row, col),
                    })
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use unindent::Unindent;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct GoL {
        rows: usize,
        cols: usize,
        lives: HashSet<(usize, usize)>,
    }
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
    fn should_create_empty_state() {
        let gol: GoL = "
            ....
            ....
            ....
            ....
            ...."
            .unindent()
            .build_gol()
            .unwrap();

        assert_eq!(GoL::empty(5, 4), gol);
    }

    #[test]
    fn should_create_void_state() {
        let gol: GoL = "".build_gol().unwrap();

        assert_eq!(GoL::empty(0, 0), gol);
    }

    #[test]
    fn should_create_no_trivial_state() {
        let gol: GoL = "
            ..*.
            .+..
            ....
            ..**
            ++.."
            .unindent()
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

    #[test]
    fn should_get_space_as_empty() {
        let gol: GoL = "
            ..*
            .+
            .  .
            ..**
            ++  "
            .unindent()
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
}
