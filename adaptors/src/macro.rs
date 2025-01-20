#[macro_use]
macro_rules! count_impls {
    ($($t:ty),*) => {{
        let mut count = 0;
        $(
            let _: &$t; // Just to ensure the type implements MyTrait
            count += 1;
        )*
        count
    }};
}
