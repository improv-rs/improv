macro_rules! break_if {
    ($cond:expr) => {
        if $cond {
            break;
        }
    };
}

macro_rules! return_if {
    ($cond:expr) => {
        if $cond {
            return;
        }
    };
}
