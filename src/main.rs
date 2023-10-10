use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug)]
enum CellState {
    E, // empty
    O, // O
    X, // X
    D, // dead cell (not affect to other cells)
}

const CELL_STATE: [CellState; 3] = [CellState::E, CellState::O, CellState::X];

#[derive(PartialEq, Clone, Copy)]
struct GridNotEndState {
    a: CellState,
    b: CellState,
    c: CellState,
    d: CellState,
    e: CellState,
    f: CellState,
    g: CellState,
    h: CellState,
    i: CellState,
}

#[derive(PartialEq, Clone, Copy)]
enum GridState {
    O,
    X,
    E(GridNotEndState),
}

fn grid_state(state: GridNotEndState) -> GridState {
    macro_rules! grid_state {
        ($state:expr, $a:ident, $b:ident, $c:ident) => {
            if $state.$a == CellState::O && $state.$a == $state.$b && $state.$a == $state.$c {
                return GridState::O;
            }
            if $state.$a == CellState::X && $state.$a == $state.$b && $state.$a == $state.$c {
                return GridState::X;
            }
        };
    }

    grid_state!(state, a, b, c);
    grid_state!(state, d, e, f);
    grid_state!(state, g, h, i);
    grid_state!(state, a, d, g);
    grid_state!(state, b, e, h);
    grid_state!(state, c, f, i);
    grid_state!(state, a, e, i);
    grid_state!(state, c, e, g);
    GridState::E(state)
}

fn is_possible_state(state: GridNotEndState) -> bool {
    macro_rules! is_possible_state {
        ($state:expr, $($field:ident),*) => {{
            $(
                if $state.$field != CellState::E {
                    let mut tmp = $state;
                    tmp.$field = CellState::E;
                    if matches!(grid_state(tmp), GridState::O) || matches!(grid_state(tmp), GridState::X) {
                        return false;
                    }
                }
            )*
            true
        }};
    }
    is_possible_state!(state, a, b, c, d, e, f, g, h, i)
}

fn is_dead_row(a: CellState, b: CellState, c: CellState) -> bool {
    let has_o = a == CellState::O
        || a == CellState::D
        || b == CellState::O
        || b == CellState::D
        || c == CellState::O
        || c == CellState::D;
    let has_x = a == CellState::X
        || a == CellState::D
        || b == CellState::X
        || b == CellState::D
        || c == CellState::X
        || c == CellState::D;
    has_o && has_x
}

fn calculate_dead_cell(state: GridNotEndState) -> GridState {
    let mut result = state;
    macro_rules! check_dead1 {
        ($state:expr, $cell:ident, $($cells:ident),*) => {
            let mut all_dead = true;
            $(
                all_dead = all_dead && $state.$cells != CellState::E;
            )*
            if all_dead {
                $state.$cell = CellState::D;
            }
        };
    }
    macro_rules! check_dead2 {
        ($state:expr, $cell:ident, $a:ident, $b:ident) => {{
            $state.$cell != CellState::E && is_dead_row($state.$cell, $state.$a, $state.$b)
        }};
    }
    check_dead1!(result, a, a, b, c, d, e, g, i);
    if check_dead2!(result, a, b, c)
        && check_dead2!(result, a, d, g)
        && check_dead2!(result, a, e, i)
    {
        result.a = CellState::D;
    }
    check_dead1!(result, b, a, b, c, e, h);
    if check_dead2!(result, b, a, c) && check_dead2!(result, b, e, h) {
        result.b = CellState::D;
    }
    check_dead1!(result, c, a, b, c, e, f, g, i);
    if check_dead2!(result, c, a, b)
        && check_dead2!(result, c, f, i)
        && check_dead2!(result, c, e, g)
    {
        result.c = CellState::D;
    }
    check_dead1!(result, d, a, d, e, f, g);
    if check_dead2!(result, d, e, f) && check_dead2!(result, d, a, g) {
        result.d = CellState::D;
    }
    check_dead1!(result, e, a, b, c, d, e, f, g, h, i);
    if check_dead2!(result, e, d, f)
        && check_dead2!(result, e, b, h)
        && check_dead2!(result, e, a, i)
        && check_dead2!(result, e, c, g)
    {
        result.e = CellState::D;
    }
    check_dead1!(result, f, c, d, e, f, i);
    if check_dead2!(result, f, d, e) && check_dead2!(result, f, c, i) {
        result.f = CellState::D;
    }
    check_dead1!(result, g, a, c, d, e, g, h, i);
    if check_dead2!(result, g, h, i)
        && check_dead2!(result, g, a, d)
        && check_dead2!(result, g, c, e)
    {
        result.g = CellState::D;
    }
    check_dead1!(result, h, b, e, g, h, i);
    if check_dead2!(result, h, g, i) && check_dead2!(result, h, b, e) {
        result.h = CellState::D;
    }
    check_dead1!(result, i, a, c, e, f, g, h, i);
    if check_dead2!(result, i, g, h)
        && check_dead2!(result, i, c, f)
        && check_dead2!(result, i, a, e)
    {
        result.i = CellState::D;
    }
    grid_state(result)
}

impl Display for GridNotEndState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i
        )?;
        Ok(())
    }
}

fn main() {
    for a in CELL_STATE {
        for b in CELL_STATE {
            for c in CELL_STATE {
                for d in CELL_STATE {
                    for e in CELL_STATE {
                        for f in CELL_STATE {
                            for g in CELL_STATE {
                                for h in CELL_STATE {
                                    for i in CELL_STATE {
                                        let state = GridNotEndState {
                                            a,
                                            b,
                                            c,
                                            d,
                                            e,
                                            f,
                                            g,
                                            h,
                                            i,
                                        };
                                        if !is_possible_state(state) {
                                            continue;
                                        }
                                        let grid_state = calculate_dead_cell(state);
                                        match grid_state {
                                            GridState::E(calculated_state) => {
                                                println!("{}={}", state, calculated_state);
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
