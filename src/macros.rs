#[macro_export]
macro_rules! static_cast_i8 {
    ($v:expr) => {
        std::mem::transmute::<_, i8>($v)
    };
}

#[macro_export]
macro_rules! static_cast_i32 {
    ($v:expr) => {
        std::mem::transmute::<_, i32>($v)
    };
}

#[macro_export]
macro_rules! static_cast_u32 {
    ($v:expr) => {
        std::mem::transmute::<_, u32>($v)
    };
}

#[macro_export]
macro_rules! static_cast_i64 {
    ($v:expr) => {
        std::mem::transmute::<_, i64>($v)
    };
}

#[macro_export]
macro_rules! static_cast_u64 {
    ($v:expr) => {
        std::mem::transmute::<_, u64>($v)
    };
}

#[macro_export]
macro_rules! static_cast_str {
    ($v:expr) => {
        std::mem::transmute::<_, &str>($v)
    };
}

#[macro_export]
macro_rules! static_cast_slice {
    ($v:expr) => {
        std::mem::transmute::<_, &[u8]>($v)
    };
}

#[macro_export]
macro_rules! check {
    ($parser:ident, $index:expr, $b:expr) => {
        unsafe {
            let s = *$parser.structurals.get_unchecked($index) as usize;
            if *$parser.src.get_unchecked(s) != $b {
                return None;
            }
        }
    };
}
