use std::time::Duration;
use std::{collections::BTreeMap, sync::RwLock};

use cons::{list, unlist, List};
use cons_char_image::list::ConsListFormat;
use deebs::{Database, DatabaseInsertList, DatabaseMapView, DatabaseMapViewMut, View};
use task_graph::task_graph;

type IVec2 = Vec2<isize>;
type UVec2 = Vec2<usize>;

#[derive(Debug, Default, Copy, Clone)]
struct Vec2<T>(T, T);

#[derive(Debug, Default, Copy, Clone)]
struct Ball;

#[derive(Debug, Default, Copy, Clone)]
struct Paddle;

#[derive(Debug, Default, Copy, Clone)]
struct Position(IVec2);

#[derive(Debug, Default, Copy, Clone)]
struct Velocity(IVec2);

#[derive(Debug, Default, Copy, Clone)]
struct Radius(usize);

#[derive(Debug, Default, Copy, Clone)]
struct Size(UVec2);

#[async_std::main]
async fn main() {
    type Table<T> = RwLock<BTreeMap<usize, RwLock<T>>>;
    type DB = Database<
        List![
            Table<Ball>,
            Table<Paddle>,
            Table<Position>,
            Table<Velocity>,
            Table<Radius>,
            Table<Size>
        ],
        List![View<List![Velocity, Position]>],
    >;

    let database = DB::default();

    database.insert_list(
        0,
        list![Ball, Position(Vec2(0, 0)), Velocity(Vec2(-4, 0)), Radius(8)],
    );

    database.insert_list(
        1,
        list![
            Paddle,
            Position(Vec2(-30, 0)),
            Velocity(Vec2(0, 0)),
            Size(Vec2(8, 16))
        ],
    );

    database.insert_list(
        2,
        list![
            Paddle,
            Position(Vec2(30, 0)),
            Velocity(Vec2(0, 0)),
            Size(Vec2(8, 16))
        ],
    );

    async fn position_integrator(db: &DB) {
        db.map_view_mut::<List![Velocity], List![Position]>(|_, ref_row, mut_row| {
            unlist!(ref_row => velocity);
            unlist!(mut_row => position);

            let Velocity(Vec2(vx, vy)) = velocity;
            let Position(Vec2(px, py)) = position;

            *position = Position(Vec2(*px + *vx, *py + *vy));
        });
    };

    async fn display_output(db: &DB) {
        db.map_view::<List![Velocity, Position]>(|_, row| {
            println!("{:?}", row.format());
        });
    }

    loop {
        let tg = task_graph! {
                [position_integrator(&database), display_output(&database)]
        };

        tg.await;

        async_std::task::sleep(Duration::from_secs_f64(0.25)).await;
    }
}
