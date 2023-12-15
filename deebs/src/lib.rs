use cons::{
    cell::{Cons, ConsCell},
    single::{ConsSingle, Single},
};
use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData, sync::RwLock};

mod database;
mod row_lock;
mod table_collection;
mod table_guard;
mod table_list;
mod table_lock;
mod view;
mod async_sandbox;

pub use database::*;
pub use row_lock::*;
pub use table_collection::*;
pub use table_guard::*;
pub use table_list::*;
pub use table_lock::*;
pub use view::*;
pub use async_sandbox::*;

/// Cast an immutable reference between lifetimes
///
/// # Safety
///
/// This function is intended for resolving complex lifetime constraints in generic traits,
/// don't call it unless you're certain that the compiler-contested borrows will remain valid
pub unsafe fn cast_lifetime<'a, 'b, T>(t: &'a T) -> &'b T {
    let ptr: *const T = t;
    ptr.as_ref().unwrap()
}

/// Cast a mutable reference between lifetimes
///
/// # Safety
///
/// This function is intended for resolving complex lifetime constraints in generic traits,
/// don't call it unless you're certain that the compiler-contested borrows will remain valid
pub unsafe fn cast_lifetime_mut<'a, 'b, T>(t: &'a mut T) -> &'b mut T {
    let ptr: *mut T = t;
    ptr.as_mut().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::BTreeMap, sync::RwLock};

    use cons::{list, unlist, List};

    #[test]
    fn test_deebs() {
        type Table<T> = RwLock<BTreeMap<usize, RwLock<T>>>;

        let database = Database::<
            List![Table<i32>, Table<f64>, Table<char>, Table<String>],
            List![View<List![f64, String]>, View<List![char, String]>],
        >::default();

        database.insert_list(0, list![0, 1.0, '2', String::from("3")]);
        database.insert_list(2, list![4, 5.0, '6', String::from("7")]);
        database.insert_list(4, list![8, 9.0, '0', String::from("00")]);

        database.remove_list::<List![i32, f64]>(&0);

        println!("Database: {:#?}", database);

        database.map_view_mut::<List![f64], List![String]>(
            |_, ref_row, mut_row| {
                unlist!(ref_row => float);
                unlist!(mut_row => string);
                *string = format!("{}", float);
            },
        );

        database.map_view::<List![char, String]>(|key, row| {
            unlist!(row => char, string);
            println!("Key: {:?}, Char: {:?}, String: {:?}", key, char, string);
        });
    }
}
