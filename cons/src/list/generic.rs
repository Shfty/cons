/// A struct that can be converted into a `ConsList` representation
pub trait StructIntoConsList {
    /// The target `ConsList` type
    type ConsList;

    /// Convert this struct into the corresponding `ConsList`
    fn into_cons_list(self) -> Self::ConsList;
}

impl StructIntoConsList for () {
    type ConsList = ();

    fn into_cons_list(self) -> Self::ConsList {}
}

/// A `ConsList` type that can be converted into a struct with the same internal type layout
pub trait ConsListIntoStruct<T> {
    /// Convert this `ConsList` into the corresponding struct
    fn into_struct(self) -> T;
}

impl ConsListIntoStruct<()> for () {
    fn into_struct(self) {}
}

#[cfg(test)]
mod tests {
    use crate::{cell::ConsCell, list, list::append::ConsListAppend, single::ConsSingle, List};
    use cons_proc_macros::Generic;

    use super::{ConsListIntoStruct, StructIntoConsList};

    #[derive(Generic, Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    pub struct TestA {
        int: i32,
        float: f64,
        char: char,
        str: &'static str,
    }

    #[derive(Generic, Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    pub struct TestB(i32, f64, char, &'static str);

    #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
    pub struct TestC {
        test_a: TestA,
        test_b: TestB,
    }

    #[allow(clippy::clippy::many_single_char_names)]
    impl ConsListIntoStruct<TestC>
        for List![i32, f64, char, &'static str, i32, f64, char, &'static str]
    {
        fn into_struct(self) -> TestC {
            let cdr = self;
            let (a, cdr) = cdr.into_destructure();
            let (b, cdr) = cdr.into_destructure();
            let (c, cdr) = cdr.into_destructure();
            let (d, cdr) = cdr.into_destructure();
            let (e, cdr) = cdr.into_destructure();
            let (f, cdr) = cdr.into_destructure();
            let (g, h) = cdr.into_destructure();
            TestC {
                test_a: TestA {
                    int: a.into_car(),
                    float: b.into_car(),
                    char: c.into_car(),
                    str: d.into_car(),
                },
                test_b: TestB(e.into_car(), f.into_car(), g.into_car(), h.into_car()),
            }
        }
    }

    impl StructIntoConsList for TestC {
        type ConsList = <<TestA as StructIntoConsList>::ConsList as ConsListAppend<
            <TestA as StructIntoConsList>::ConsList,
        >>::Append;

        fn into_cons_list(self) -> Self::ConsList {
            self.test_a
                .into_cons_list()
                .append(self.test_b.into_cons_list())
        }
    }

    #[test]
    fn test_cons_list_generic() {
        let named_struct = TestA {
            int: 1,
            float: 2.0,
            char: '3',
            str: "four",
        };

        let unnamed_struct = TestB(1, 2.0, '3', "four");

        let named_generic = named_struct.into_cons_list();
        let unnamed_generic = unnamed_struct.into_cons_list();

        assert!(named_generic == list![1, 2.0, '3', "four"]);
        assert!(unnamed_generic == list![1, 2.0, '3', "four"]);

        let recip_named_struct: TestA = named_generic.into_struct();
        let recip_unnamed_struct: TestB = unnamed_generic.into_struct();

        assert!(recip_named_struct == named_struct);
        assert!(recip_unnamed_struct == unnamed_struct);

        let composite = TestC {
            test_a: named_struct,
            test_b: unnamed_struct,
        };

        let composite_generic = composite.into_cons_list();
        println!("{:#?}", composite_generic);
        assert!(composite_generic == list![1, 2.0, '3', "four", 1, 2.0, '3', "four"]);

        let recip_composite_struct = composite_generic.into_struct();
        println!("{:#?}", recip_composite_struct);
        assert!(recip_composite_struct == composite);
    }
}
