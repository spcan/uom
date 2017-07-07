//! Information (base unit bit, bit).

quantity! {
    /// Information (base unit bit, bit).
    quantity: Information; "information";
    /// Information dimension, bit.
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0,     // luminous intensity
        P1>;    // information
    units {
        @yobibit: prefix!(yobi); "Yibit", "yobibit", "yobibits";
        @yottabit: prefix!(yotta); "Ybit", "yottabit", "yottabits";
        @zebibit: prefix!(zebi); "Zibit", "zebibit", "zebibits";
        @zettabit: prefix!(zetta); "Zbit", "zettabit", "zettabits";
        @exbibit: prefix!(exbi); "Eibit", "exbibit", "exbibits";
        @exabit: prefix!(exa); "Ebit", "exabit", "exabits";
        @pebibit: prefix!(pebi); "Pibit", "pebibit", "pebibits";
        @petabit: prefix!(peta); "Pbit", "petabit", "petabits";
        @tebibit: prefix!(tebi); "Tibit", "tebibit", "tebibits";
        @terabit: prefix!(tera); "Tbit", "terabit", "terabits";
        @gibibit: prefix!(gibi); "Gibit", "gibibit", "gibibits";
        @gigabit: prefix!(giga); "Gbit", "gigabit", "gigabits";
        @mebibit: prefix!(mebi); "Mibit", "mebibit", "mebibits";
        @megabit: prefix!(mega); "Mbit", "megabit", "megabits";
        @kibibit: prefix!(kibi); "Kibit", "kibibit", "kibibits";
        @kilobit: prefix!(kilo); "kbit", "kilobit", "kilobits";
        /// 
        @bit: prefix!(none) / 8.0; "bit", "bit", "bits";

        @yobibyte: prefix!(yobi) * 8.0; "YiB", "yobibyte", "yobibytes";
        @yottabyte: prefix!(yotta) * 8.0; "YB", "yottabyte", "yottabytes";
        @zebibyte: prefix!(zebi) * 8.0; "ZiB", "zebibyte", "zebibytes";
        @zettabyte: prefix!(zetta) * 8.0; "ZB", "zettabyte", "zettabytes";
        @exbibyte: prefix!(exbi) * 8.0; "EiB", "exbibyte", "exbibytes";
        @exabyte: prefix!(exa) * 8.0; "EB", "exabyte", "exabytes";
        @pebibyte: prefix!(pebi) * 8.0; "PiB", "pebibyte", "pebibytes";
        @petabyte: prefix!(peta) * 8.0; "PB", "petabyte", "petabytes";
        @tebibyte: prefix!(tebi) * 8.0; "TiB", "tebibyte", "tebibytes";
        @terabyte: prefix!(tera) * 8.0; "TB", "terabyte", "terabytes";
        @gibibyte: prefix!(gibi) * 8.0; "GiB", "gibibyte", "gibibytes";
        @gigabyte: prefix!(giga) * 8.0; "GB", "gigabyte", "gigabytes";
        @mebibyte: prefix!(mebi) * 8.0; "MiB", "mebibyte", "mebibytes";
        @megabyte: prefix!(mega) * 8.0; "MB", "megabyte", "megabytes";
        @kibibyte: prefix!(kibi) * 8.0; "KiB", "kibibyte", "kibibytes";
        @kilobyte: prefix!(kilo) * 8.0; "kB", "kilobyte", "kilobytes";
        /// 
        @byte: prefix!(none) * 8.0; "B", "byte", "bytes";
    }
}
