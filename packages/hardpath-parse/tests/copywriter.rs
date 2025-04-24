//! Uses the current Asmov Copywriter filesystem standard as test input.
//! This data should be kept synchronized with the copywriter project, manually.

const COPYWRITER_FS_INDENT: usize = 4;
const COPYWRITER_FS_NONDESC: &'static str = r"
    .
    |-- site.toml
    |-- content/
    |-- pkg/
    |-+ src/
        |-- css/
        |-- docs/
        |-- downloads/
        |-+ hbs/
        | |-- layout/
        | |-- model/
        |-- images/
        |-- js/
        |-- music/
        |-- sounds/
        |-- videos/
        |-- wasm/
";

const COPYWRITER_FS_DESC_INLINE: &'static str = r"
    Asmov Copywriter Filesystem
    Standard filesystem heirarchy for copywriter projects
    .
    |-- site.toml :: Site Information
    |   Information about the website
    |-- content/ :: Content
    |   Website content organized by datamodel
    |-- pkg/ :: Packages
    |   Third-party bundles of Src files for the website
    |-+ src/ :: Src
        | HTML, CSS, JS, multimedia, etc.
        |-- css/ :: CSS
        |   Styling and themes for the website
        |-- docs/ :: Documents
        |   PDFs, Word documents, etc.
        |-- downloads/ :: Downloads
        |   Files for download
        |-+ hbs/ :: Templates
        | | Handlebars templates for building the website
        | |-- layout/ :: Layout Templates
        | |   Templates for the website's header, footer, etc.
        | |-- model/ :: Model Templates
        |     Template pages and snippets for each datamodel
        |-- images/ :: Images
        |   Images for the website
        |-- js/ :: Javascript
        |   Javascript code for the website
        |-- music/ :: Music
        |   Music for the website
        |-- sounds/ :: Sounds
        |   Sounds for the website
        |-- videos/ :: Videos
        |   Videos for the website
        |-- wasm/ :: WebAssembly
            WebAssembly code for the website
";

const COPYWRITER_FS_DESC_OUTLINE: &'static str = r"
    Asmov Copywriter Filesystem
    Standard filesystem heirarchy for copywriter projects
    .
    |-- site.toml
    |-- content/
    |-- pkg/
    |-+ src/
        |-- css/
        |-- docs/
        |-- downloads/
        |-+ hbs/
        | |-- layout/
        | |-- model/
        |-- images/
        |-- js/
        |-- music/
        |-- sounds/
        |-- videos/
        |-- wasm/

    ./site.toml
        Site Information
        Information about the website
    ./content/
        Content
        Website content organized by datamodel
    ./pkg/
        Packages
        Third-party bundles of Src files for the website
    ./src/
        Src
        HTML, CSS, JS, multimedia, etc.
    ./src/css/
        CSS
        Styling and themes for the website
    ./src/docs/
        Documents
        PDFs, Word documents, etc.
    ./src/downloads/
        Downloads
        Files for download
    ./src/hbs/
        Templates
        Handlebars templates for building the website
    ./src/hbs/layout/
        Layout Templates
        Templates for the website's header, footer, etc.
    ./src/hbs/model/
        Model Templates
        Template pages and snippets for each datamodel
    ./src/images/
        Images
        Images for the website
    ./src/js/
        Javascript
        Javascript code for the website
    ./src/music/
        Music
        Music for the website
    ./src/sounds/
        Sounds
        Sounds for the website
    ./src/videos/
        Videos
        Videos for the website
    ./src/wasm/
        WebAssembly
        WebAssembly code for the website
";

/// Tests against three different variations of layout:
/// - Non-Descriptive: No name or description for entries
/// - Descriptive Inline: Names and descriptions are provided inline *with* each entry
/// - Descriptive Outline: Names and descriptions are provided after filestructure, with a filepath as the key
#[cfg(test)]
mod tests {
    use super::*;
    use asmov_common_hardpath_parse as parse;
    use asmov_common_hardpath_model as model;

    fn indent(s: String, len: usize) -> String {
        s.split("\n").map(|line| format!("{indent}{line}\n", indent = " ".repeat(len))).collect()
    }

    /// Convert from a string into a tree, then from that tree back into a string, then compare the results against
    /// the original string.
    #[test]
    fn test_string_conversion() {
        let nondesc: model::SoftpathTree = parse::str_to_softpath_tree(COPYWRITER_FS_NONDESC).unwrap();
        let nondesc: String = indent(parse::softpath_tree_to_string(&nondesc).unwrap(), COPYWRITER_FS_INDENT);
        assert_eq!(COPYWRITER_FS_NONDESC, nondesc, "Parsed non-descriptive schema should match original");

        let desc_inline: model::SoftpathTree = parse::str_to_softpath_tree(COPYWRITER_FS_DESC_INLINE).unwrap();
        let desc_inline: String = indent(parse::softpath_tree_to_string(&desc_inline).unwrap(), COPYWRITER_FS_INDENT);
        assert_eq!(COPYWRITER_FS_DESC_INLINE, desc_inline, "Parsed descriptive inline schema should match original");

        let desc_outline: model::SoftpathTree = parse::str_to_softpath_tree(COPYWRITER_FS_DESC_OUTLINE).unwrap();
        let desc_outline: String = indent(parse::softpath_tree_to_string(&desc_outline).unwrap(), COPYWRITER_FS_INDENT);
        assert_eq!(COPYWRITER_FS_DESC_OUTLINE, desc_outline, "Parsed descriptive outline schema should match original");
    }
}
