macro_rules! typestate {
    (
        $($quantifier:vis state $state_type:ident { $($state:ident),* })*
    ) => {
        $(
            $quantifier trait $state_type {}

            $(
                $quantifier struct $state;
                impl $state_type for $state {}
            )*
        )*
    };
}

pub(crate) use typestate;
