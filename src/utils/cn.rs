/// This macro handles conditional merging of tailwind classes based on reactive values.
///<br>
///
/// You can define classes to merge with comma separated strings.
/// ```
/// cn!("bg-red-300", "bg-blue-300")
/// ```
///<br>
///
/// You can define non-reactive conditional classes (note: these have to come first) like so:
/// ```
/// ✅ cn!((1 == 1, "bg-red-300"), "bg-blue-300")
/// ❌ cn!("bg-blue-300", (1 == 1, "bg-red-300"))
/// ```
/// <br>
///
/// You can define reactive conditional classes (note: these have to come first) like so:
/// ```
/// ✅ cn!(#(reactive_value_or_expression, "bg-red-300"), "bg-blue-300")
/// ❌ cn!("bg-blue-300", #(reactive_value_or_expression, "bg-red-300"))
/// ```
///<br>
///
/// It uses the [`tailwind_fuse`] crate under the hood for merging

#[macro_export]
macro_rules! cn {
    (($a:expr, $b:expr)) => {{
        if $a {$b} else { "" }
    }};

    (#($a:ident, $b:expr)) => {{
        || if $a.get() {$b} else { "" }
     }};

     (#($a:expr, $b:expr)) => {{
        || if $a {$b} else { "" }
     }};

    ($(#($a:ident, $b:expr)),+ $(,)? $($c:expr),+ $(,)?) => {{
        use tailwind_fuse::tw_merge;
        move || tw_merge!(
            $($c)*,
            $(if $a.get() {$b} else { "" })*
        )
    }};

    ($(#($a:expr, $b:expr)),+ $(,)? $($c:expr),+ $(,)?) => {{
        use tailwind_fuse::tw_merge;
        move || tw_merge!(
            $($c)*,
            $(cn!(($a,$b)))*,
        )
    }};

    ($(($a:expr, $b:expr)),+ $(,)? $($c:expr),+ $(,)?) => {{
        use tailwind_fuse::tw_merge;
        tw_merge!(
            $($c)*,
            $(cn!(($a,$b)))*,
        )
    }};

    ($($arg:expr),+ $(,)?) => {{
        use tailwind_fuse::tw_merge;
        tw_merge!($($arg,)*)
    }};

    // TODO consider making it cn!("base-class", (conditional, "class")) since that is the final merge order
}

#[cfg(test)]
mod cn_tests {
    use leptos::prelude::*;

    #[test]
    fn macro_syntax_works() {
        let reactive_bool = RwSignal::new(true);
        let reactive_value = RwSignal::new(1);

        assert_eq!(cn!("a"), "a");
        assert_eq!(cn!("a", "b"), "a b");

        assert_eq!(cn!((true, "a")), "a");
        assert_eq!(cn!((true, "a"), "b"), "b a");
        assert_eq!(cn!((false, "a"), "b"), "b");
        assert_eq!(cn!((1 == 1, "a"), "b"), "b a");

        // Reactive value tests
        assert_eq!(cn!(#(reactive_bool, "a"))(), "a");
        assert_eq!(cn!(#(reactive_bool, "a"), "b")(), "b a");
        assert_eq!(cn!(#(reactive_value.get() == 1, "a"), "b")(), "b a");

        reactive_bool.set(false);

        assert_eq!(cn!(#(reactive_bool, "a"), "b")(), "b");

        assert_eq!(cn!(#(reactive_bool, "a"), "b")(), "b");
    }
}
