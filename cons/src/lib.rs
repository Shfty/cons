pub mod alist;
pub mod array;
pub mod cell;
pub mod list;
pub mod plist;
pub mod tree;
pub mod folder;
pub mod index;
pub mod mapper;
pub mod single;

#[cfg(test)]
mod tests {
    #[test]
    fn test_list_proc_macro() {
        let list_literal = cons_proc_macros::list![1, 2, 3, 4];
        println!("List: {:?}", list_literal);
        let list_sized = cons_proc_macros::list![256; 5];
        println!("List: {:?}", list_sized);
    }
}
