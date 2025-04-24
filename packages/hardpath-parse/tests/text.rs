#[cfg(test)]
mod tests {
    /// Uses the current Asmov Copywriter filesystem standard as input.
    /// Tests against three different variations of layout:
    /// - Non-Descriptive: No name or description for entries
    /// - Descriptive Inline: Names and descriptions are provided inline *with* each entry
    /// - Descriptive Outline: Names and descriptions are provided after filestructure, with a filepath as the key
    #[test]
    fn test_copywriter_fs() {
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
              |   Image files for the website
              |-- js/ :: Javascript
              |   Javascript code for the website
              |-- music/ :: Music
              |   Music for the website
              |-- sounds/ :: Sounds
              |   Sound files for the website
              |-- videos/ :: Videos
              |   Video files for the website
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

        ";


    }
}
