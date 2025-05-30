#[macro_export]
macro_rules! cn_inner {
    (($a:expr,$($b:tt)+)) => ({
        if $a {cn_inner!($($b)*)} else {""}
    });

    (($a:expr,$($b:tt)+),$($tail:tt)+) => ({
        cn_inner!(
            if $a {cn_inner!($($b)*)} else {""},
            $($tail)*
        )
    });

    ($a:expr,$($tail:tt)+) => {{
        tw_merge!($a,cn_inner!($($tail)*))
     }};

    ($a:expr) => {{
       $a
    }};
}

/// This macro handles conditional merging of tailwind classes based on reactive values.
///<br>
///
/// You can define classes to merge with comma separated strings.
/// ```
/// cn!("bg-red-300", "bg-blue-300")
/// ```
///<br>
///
/// You can define a conditional in parentheses classes like so:
/// ```
/// cn!("bg-blue-300", (1 == 1, "bg-red-300"))
/// ```
/// <br>
///
/// You can make the entire defnition reactive by doing cn!(#(...)) like so:
/// ```
/// cn!(#("bg-blue-300", (bool_signal(), "bg-red-300")));
/// cn!(#("bg-blue-300", (num_signal() == 1, "bg-red-300")));
/// ```
///<br>
///
/// It uses the [`tailwind_fuse`] crate under the hood for merging
#[macro_export]
macro_rules! cn {
    (#($($tail:tt)+)) => ({
        use tailwind_fuse::tw_merge;
        use crate::cn_inner;

        move || tw_merge!(
            cn_inner!($($tail)*)
        )
    });
    ($($tail:tt)+) => ({
        use tailwind_fuse::tw_merge;
        use crate::cn_inner;

        tw_merge!(
            cn_inner!($($tail)*)
        )
    });
}

#[cfg(test)]
mod cn_tests {
    use leptos::prelude::*;

    #[test]
    fn macro_syntax_works() {
        let non_reactive_bool = true;
        let non_reactive_value = 1;
        let reactive_bool = RwSignal::new(true);
        let reactive_value = RwSignal::new(1);

        assert_eq!(cn!((non_reactive_bool, "a")), "a");
        assert_eq!(cn!("a", "b", (non_reactive_bool, "c")), "a b c");
        assert_eq!(cn!((non_reactive_value == 1, "a"), "b"), "a b");
        assert_eq!(
            cn!((non_reactive_bool, "a"), (reactive_bool(), "b"), "c"),
            "a b c"
        );
        assert_eq!(cn!("a", (non_reactive_bool, "b"), "c"), "a b c");
        assert_eq!(cn!((non_reactive_bool, (non_reactive_bool, "a"))), "a");

        // // Reactive value tests
        assert_eq!(cn!(#((reactive_bool(), "a")))(), "a");
        assert_eq!(cn!(#("a", "b", (reactive_bool(), "c")))(), "a b c");
        assert_eq!(cn!(#((reactive_value.get() == 1, "a"), "b"))(), "a b");
        assert_eq!(
            cn!(#((reactive_bool(), "a"), (reactive_bool(), "b"), "c"))(),
            "a b c"
        );
        assert_eq!(cn!(#("a", (reactive_bool(), "b"), "c"))(), "a b c");
        assert_eq!(
            cn!(#(
                (reactive_bool(), (reactive_bool(), "a"))
            ))(),
            "a"
        );

        // TODO - Fix this type error that this test throws

        // assert_eq!(
        //     cn!(#(
        //         (reactive_bool(), (reactive_bool(), "a"), "b")
        //     ))(),
        //     "a b"
        // );

        reactive_bool.set(false);

        assert_eq!(cn!(#((reactive_bool(), "a"), "b"))(), "b");
    }
}
