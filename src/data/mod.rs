//! 

/// 
#[macro_use]
pub mod information {
    quantity! {
        /// 
        quantity: Information; "information";
        /// 
        dimension: Q<P1>;
        units {
            @yobibyte: prefix!(yobi); "YiB", "yobibyte", "yobibytes";
            @yottabyte: prefix!(yotta); "YB", "yottabyte", "yottabytes";
            @zebibyte: prefix!(zebi); "ZiB", "zebibyte", "zebibytes";
            @zettabyte: prefix!(zetta); "ZB", "zettabyte", "zettabytes";
            @exbibyte: prefix!(exbi); "EiB", "exbibyte", "exbibytes";
            @exabyte: prefix!(exa); "EB", "exabyte", "exabytes";
            @pebibyte: prefix!(pebi); "PiB", "pebibyte", "pebibytes";
            @petabyte: prefix!(peta); "PB", "petabyte", "petabytes";
            @tebibyte: prefix!(tebi); "TiB", "tebibyte", "tebibytes";
            @terabyte: prefix!(tera); "TB", "terabyte", "terabytes";
            @gibibyte: prefix!(gibi); "GiB", "gibibyte", "gibibytes";
            @gigabyte: prefix!(giga); "GB", "gigabyte", "gigabytes";
            @mebibyte: prefix!(mebi); "MiB", "mebibyte", "mebibytes";
            @megabyte: prefix!(mega); "MB", "megabyte", "megabytes";
            @kibibyte: prefix!(kibi); "KiB", "kibibyte", "kibibytes";
            @kilobyte: prefix!(kilo); "kB", "kilobyte", "kilobytes";
            /// 
            @byte: prefix!(none); "B", "byte", "bytes";

            @yobibit: prefix!(yobi) / 8.0; "Yibit", "yobibit", "yobibits";
            @yottabit: prefix!(yotta) / 8.0; "Ybit", "yottabit", "yottabits";
            @zebibit: prefix!(zebi) / 8.0; "Zibit", "zebibit", "zebibits";
            @zettabit: prefix!(zetta) / 8.0; "Zbit", "zettabit", "zettabits";
            @exbibit: prefix!(exbi) / 8.0; "Eibit", "exbibit", "exbibits";
            @exabit: prefix!(exa) / 8.0; "Ebit", "exabit", "exabits";
            @pebibit: prefix!(pebi) / 8.0; "Pibit", "pebibit", "pebibits";
            @petabit: prefix!(peta) / 8.0; "Pbit", "petabit", "petabits";
            @tebibit: prefix!(tebi) / 8.0; "Tibit", "tebibit", "tebibits";
            @terabit: prefix!(tera) / 8.0; "Tbit", "terabit", "terabits";
            @gibibit: prefix!(gibi) / 8.0; "Gibit", "gibibit", "gibibits";
            @gigabit: prefix!(giga) / 8.0; "Gbit", "gigabit", "gigabits";
            @mebibit: prefix!(mebi) / 8.0; "Mibit", "mebibit", "mebibits";
            @megabit: prefix!(mega) / 8.0; "Mbit", "megabit", "megabits";
            @kibibit: prefix!(kibi) / 8.0; "Kibit", "kibibit", "kibibits";
            @kilobit: prefix!(kilo) / 8.0; "kbit", "kilobit", "kilobits";
            /// 
            @bit: prefix!(none) / 8.0; "bit", "bit", "bits";
        }
    }
}

system! {
    /// 
    quantities: Q {
        information: byte, B;
    }

    /// 
    units: U {
        Information,
    }
}

/// [`Quantity`](struct.Quantity.html) type aliases using `f32` as the underlying
/// storage type.
#[cfg(feature = "f32")]
pub mod f32 {
    Q!(data, f32);
}

/// [`Quantity`](struct.Quantity.html) type aliases using `f64` as the underlying
/// storage type.
#[cfg(feature = "f64")]
pub mod f64 {
    Q!(data, f64);
}
