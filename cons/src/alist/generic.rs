pub trait StructToConsAList {
    type ConsAList;

    fn into_cons_alist(self) -> Self::ConsAList;
}

impl StructToConsAList for () {
    type ConsAList = ();

    fn into_cons_alist(self) -> Self::ConsAList {}
}

pub trait ConsAListToStruct<T> {
    fn into_struct(self) -> T;
}

impl ConsAListToStruct<()> for () {
    fn into_struct(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    struct Test {
        int: i32,
        float: f64,
        char: char,
        str: &'static str,
    }

    #[allow(clippy::clippy::type_complexity)]
    impl StructToConsAList for Test {
        type ConsAList = (
            (&'static str, i32),
            (
                (&'static str, f64),
                ((&'static str, char), ((&'static str, &'static str), ())),
            ),
        );

        fn into_cons_alist(self) -> Self::ConsAList {
            (
                ("int", self.int),
                (
                    ("float", self.float),
                    (("char", self.char), (("str", self.str), ())),
                ),
            )
        }
    }

    impl ConsAListToStruct<Test>
        for (
            (&'static str, i32),
            (
                (&'static str, f64),
                ((&'static str, char), ((&'static str, &'static str), ())),
            ),
        )
    {
        fn into_struct(self) -> Test {
            let ((_, int), ((_, float), ((_, char), ((_, str), ())))) = self;

            Test {
                int,
                float,
                char,
                str,
            }
        }
    }

    #[test]
    fn test_cons_alist_generic() {
        let test_struct = Test {
            int: 1,
            float: 2.0,
            char: '3',
            str: "four",
        };

        let cons_alist = test_struct.into_cons_alist();

        let recip_struct = cons_alist.into_struct();

        assert!(test_struct == recip_struct);
    }
}
