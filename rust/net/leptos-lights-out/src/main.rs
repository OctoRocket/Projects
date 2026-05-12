use leptos::{ev::MouseEvent, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| LightsOut);
}

type Coords = (i32, i32);

#[component]
fn LightsOut() -> impl IntoView {
    view! {
        <Grid height=5 width=5 />
    }
}

#[derive(Debug, Clone)]
struct TileData {
    on: RwSignal<bool>,
    adjacencies: Vec<Coords>
}

impl TileData {
    fn new(coords: Coords, state: bool, upper_bounds: Coords) -> Self {
        let mut adjacencies = vec![];
        for set in [(0, 1), (1, 0), (0, -1), (-1, 0)].map(|c| (coords.0 + c.0, coords.1 + c.1)) {
            if set.0 >= 0 && set.1 >= 0 && set.0 < upper_bounds.0 && set.1 < upper_bounds.1 {
                adjacencies.push(set);
            }
        }

        Self {
            on: RwSignal::new(state),
            adjacencies,
        }
    }
}

#[component]
fn Tile(tile: TileData, grid_signal: WriteSignal<Vec<Vec<TileData>>>) -> impl IntoView {
    view!{
        <div
            class="square"
            style:background-color=move || if tile.on.get() { "white" } else { "black" }
            on:click=move |_| tile.on.update(|s| {
                *s = !*s;
                for adjacent in &tile.adjacencies {
                    grid_signal.update(|g| g[adjacent.1 as usize][adjacent.0 as usize].on.update(|s| *s = !*s))
                }
            })
        ></div>
    }
}

#[component]
fn Grid(height: usize, width: usize) -> impl IntoView {
    let (grid,  set_grid) = signal(vec![]);
    for h_coord in 0..height {
        let mut row = vec![];
        for w_coord in 0..width {
            row.push(TileData::new((w_coord as i32, h_coord as i32), rand::random_bool(0.5), (width as i32, height as i32)));
        }
        set_grid.update(|rows| rows.push(row));
    }

    view! {
        <div class="top-to-bottom">
        {grid.read().iter().map(|row| {
            view! {
                <div class="left-to-right">
                    {row.iter().map(|t| Tile(TileProps { tile: t.clone(), grid_signal: set_grid })).collect_view()}
                </div>
            }
        }).collect_view()}
        </div>
    }
}
