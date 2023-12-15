pub trait StructToConsPList {
    type ConsPList;

    fn into_cons_plist(self) -> Self::ConsPList;
}

impl StructToConsPList for () {
    type ConsPList = ();

    fn into_cons_plist(self) -> Self::ConsPList {}
}

pub trait ConsPListToStruct<T> {
    fn into_struct(self) -> T;
}

impl ConsPListToStruct<()> for () {
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
    impl StructToConsPList for Test {
        type ConsPList = (
            &'static str,
            (
                i32,
                (
                    &'static str,
                    (
                        f64,
                        (&'static str, (char, (&'static str, (&'static str, ())))),
                    ),
                ),
            ),
        );

        fn into_cons_plist(self) -> Self::ConsPList {
            (
                "int",
                (
                    self.int,
                    (
                        "float",
                        (self.float, ("char", (self.char, ("str", (self.str, ()))))),
                    ),
                ),
            )
        }
    }

    impl ConsPListToStruct<Test>
        for (
            &'static str,
            (
                i32,
                (
                    &'static str,
                    (
                        f64,
                        (&'static str, (char, (&'static str, (&'static str, ())))),
                    ),
                ),
            ),
        )
    {
        fn into_struct(self) -> Test {
            let (_, (int, (_, (float, (_, (char, (_, (str, ())))))))) = self;

            Test {
                int,
                float,
                char,
                str,
            }
        }
    }

    #[test]
    fn test_cons_plist_generic() {
        let test_struct = Test {
            int: 1,
            float: 2.0,
            char: '3',
            str: "four",
        };

        let cons_plist = test_struct.into_cons_plist();

        let recip_struct = cons_plist.into_struct();

        assert!(test_struct == recip_struct);
    }
}
