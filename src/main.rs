use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum CellState {
    E, // empty
    O, // O
    X, // X
    D, // dead cell (not affect to other cells)
}

const CELL_STATE: [CellState; 3] = [CellState::E, CellState::O, CellState::X];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

fn mark_all_as_dead(state: GridNotEndState) -> GridNotEndState {
    macro_rules! mark_all_as_dead {
        ($state:expr, $($field:ident),*) => {{
            let mut result = $state;
            $(
                if $state.$field != CellState::E {
                    result.$field = CellState::D;
                }
            )*
            result
        }};
    }
    mark_all_as_dead!(state, a, b, c, d, e, f, g, h, i)
}

fn calculate_dead_cell(state: GridNotEndState) -> GridState {
    match grid_state(state) {
        GridState::O => {
            return GridState::O;
        }
        GridState::X => {
            return GridState::X;
        }
        _ => (),
    }
    let mut tmp = state;
    if state.a == CellState::E {
        tmp.a = CellState::O;
    }
    if state.b == CellState::E {
        tmp.b = CellState::O;
    }
    if state.c == CellState::E {
        tmp.c = CellState::O;
    }
    if state.d == CellState::E {
        tmp.d = CellState::O;
    }
    if state.e == CellState::E {
        tmp.e = CellState::O;
    }
    if state.f == CellState::E {
        tmp.f = CellState::O;
    }
    if state.g == CellState::E {
        tmp.g = CellState::O;
    }
    if state.h == CellState::E {
        tmp.h = CellState::O;
    }
    if state.i == CellState::E {
        tmp.i = CellState::O;
    }
    let tmp_o = !matches!(grid_state(tmp), GridState::O);
    if state.a == CellState::E {
        tmp.a = CellState::X;
    }
    if state.b == CellState::E {
        tmp.b = CellState::X;
    }
    if state.c == CellState::E {
        tmp.c = CellState::X;
    }
    if state.d == CellState::E {
        tmp.d = CellState::X;
    }
    if state.e == CellState::E {
        tmp.e = CellState::X;
    }
    if state.f == CellState::E {
        tmp.f = CellState::X;
    }
    if state.g == CellState::E {
        tmp.g = CellState::X;
    }
    if state.h == CellState::E {
        tmp.h = CellState::X;
    }
    if state.i == CellState::E {
        tmp.i = CellState::X;
    }
    let tmp_x = !matches!(grid_state(tmp), GridState::X);
    if tmp_o && tmp_x {
        return grid_state(mark_all_as_dead(state));
    }

    let mut result = state;
    macro_rules! check_dead {
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
    check_dead!(result, a, a, b, c, d, e, g, i);
    check_dead!(result, b, a, b, c, e, h);
    check_dead!(result, c, a, b, c, e, f, g, i);
    check_dead!(result, d, a, d, e, f, g);
    check_dead!(result, e, a, b, c, d, e, f, g, h, i);
    check_dead!(result, f, c, d, e, f, i);
    check_dead!(result, g, a, c, d, e, g, h, i);
    check_dead!(result, h, b, e, g, h, i);
    check_dead!(result, i, a, c, e, f, g, h, i);
    grid_state(result)
}

fn main() {
    let mut set = HashSet::new();
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
                                        let state = calculate_dead_cell(state);
                                        if set.contains(&state) {
                                            continue;
                                        }
                                        set.insert(state);
                                        println!("{:?}", state);
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
