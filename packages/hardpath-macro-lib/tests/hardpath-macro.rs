#[cfg(test)]
mod tests {
    use quote::quote;
    use asmov_common_hardpath_macro_lib as lib;

    #[test]
    fn test_hardpath_macro() {
        let input = quote!{
            /// This is comment line 1
            /// This is comment line 2
            /// ```hardpath
            /// This is comment line 3
            ///     This is comment line 4
            /// ```
            ///
            /// This is comment line 5
            const INPUT_FILETREE: HardpathTree = hardpath!();
        };

        let _hardpath = lib::parse_hardpath_macro(input).unwrap();
    }
}
