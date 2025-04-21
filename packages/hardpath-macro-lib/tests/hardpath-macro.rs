#[cfg(test)]
mod tests {
    use quote::quote;
    use asmov_common_hardpath_macro_lib as lib;

    #[test]
    fn test_hardpath_macro() {
        let input = quote!{
            /// This is doc line title
            /// This is doc line subline
            ///
            /// This is doc line errata 1
            ///
            /// This is doc line errata 2
            /// ```hardpath
            /// .
            /// |-- dir1/
            /// |-+ dir2/
            /// | |-- file2-1
            /// | |-- file2-2.txt
            /// | |-- dir2-1/
            /// |-+ dir3/
            /// | |-- dir3-1/
            /// |-+ dir4/
            /// | |-- dir4-1/
            /// | |-+ dir4-2/
            /// | |-+ dir4-2-1/
            /// |   |-+ dir4-2-1-1/
            /// |     |-+ dir4-2-1-1-1/
            /// |       |-+ dir4-2-1-1-1-1/
            /// |         |-+ dir4-2-1-1-1-1-1/
            /// |           |-- dir4-2-1-1-1-1-1-1/
            /// |           |-+ dir4-2-1-1-1-1-1-2/
            /// |             |-- file4-2-1-1-1-1-1-2-1.dat
            /// |-- file1
            /// |-- file2.txt
            /// ```
            ///
            /// This is doc line post errata 1
            /// This is doc line post errata 1
            ///
            struct InputFileTree;
        };

        let _hardpath = lib::parse_hardpath_macro(input).unwrap();
    }
}
