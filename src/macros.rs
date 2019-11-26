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
macro_rules! read {
    ($parser:ident, $src:expr, $b:expr) => {
        match $parser.structurals.get($parser.structurals_i) {
            Some(src_i) => {
                if $src[*src_i as usize] != $b {
                    return Err(());
                }
            }
            None => {
                return Err(());
            }
        };
    };
}

#[macro_export]
macro_rules! read_until {
    ($parser:ident, $src:expr, $b:expr) => {
        loop {
            match $parser.structurals.get($parser.structurals_i) {
                Some(src_i) => {
                    if $src[*src_i as usize] != $b {
                        $parser.structurals_i += 1;
                        continue;
                    }
                    break;
                }
                None => {
                    return Err(());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! bump {
    ($parser:ident) => {
        $parser.structurals_i += 1;
    };
}

#[macro_export]
macro_rules! index {
    ($parser:ident) => {
        match $parser.structurals.get($parser.structurals_i) {
            Some(src_i) => *src_i as usize,
            None => {
                return Err(());
            }
        };
    };
}
