# UNIC: Unicode and Internationalization Crates for Rust


## to update versions

1. download latest files from:
 * https://unicode.org/Public/emoji/latest/

`wget -P external/unicode/emoji/data/ -A .txt -nd -r -l1 --no-parent https://unicode.org/Public/emoji/latest/`
 * https://unicode.org/Public/idna/latest/

`wget -P external/unicode/idna/data/ -A .txt -nd -r -l1 --no-parent https://unicode.org/Public/idna/latest/`
 * https://unicode.org/Public/emoji/latest/


`wget -P external/unicode/ucd https://unicode.org/Public/UCD/latest/ucd/UCD.zip && unzip external/unicode/ucd/UCD.zip -d external/unicode/ucd/data && wget -P external/unicode/ucd https://unicode.org/Public/UCD/latest/ucd/Unihan.zip && unzip external/unicode/ucd/Unihan.zip -d external/unicode/ucd/data/Unihan`