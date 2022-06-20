// Copyright 2022 Oxide Computer Company

use serde::Serialize;

// Process the configuration options lifted from the rustfmt repo. Note that
// the default value is ignored, but left in to simplify updates.
macro_rules! create_config {
    ($($i:ident: $ty:ty, $def:expr, $stb:expr, $( $dstring:expr ),+ );+ $(;)*) => {
        /// `rustfmt` configuration.
        ///
        /// See the [`rustfmt` documentation](https://rust-lang.github.io/rustfmt)
        /// for the descriptions of these stable and non-stable options.
        #[derive(Serialize, Default)]
        pub struct Config {
            $(
                $(
                    #[doc = $dstring]
                )*
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $i: Option<$ty>,
            )*
        }

        impl Config {
            pub(crate) fn unstable(&self) -> bool {
                false
                $(
                    // Not stable and explicitly set.
                    || (!$stb && self.$i.is_some())
                )*
            }

            pub(crate) fn list_unstable(&self) -> String {
                let mut list = Vec::new();
                $(
                    if !$stb && self.$i.is_some() {
                        list.push(stringify!($i));
                    }
                )*
                list.join(", ")
            }
        }
    };
}

// Per https://github.com/rust-lang/rustfmt/blob/master/src/config/mod.rs
// ... skipping the section titled "Not user-facing" and "Control options".
//
// This macro defines configuration options used in rustfmt. Each option
// is defined as follows:
//
// `name: value type, default value, is stable, description;`
create_config! {
    // Fundamental stuff
    max_width: usize, 100, true, "Maximum width of each line";
    hard_tabs: bool, false, true, "Use tab characters for indentation, spaces for alignment";
    tab_spaces: usize, 4, true, "Number of spaces per tab";
    newline_style: NewlineStyle, NewlineStyle::Auto, true, "Unix or Windows line endings";
    indent_style: IndentStyle, IndentStyle::Block, false, "How do we indent expressions or items";

    // Width Heuristics
    use_small_heuristics: Heuristics, Heuristics::Default, true, "Whether to use different \
        formatting for items and expressions if they satisfy a heuristic notion of 'small'";
    fn_call_width: usize, 60, true, "Maximum width of the args of a function call before \
        falling back to vertical formatting.";
    attr_fn_like_width: usize, 70, true, "Maximum width of the args of a function-like \
        attributes before falling back to vertical formatting.";
    struct_lit_width: usize, 18, true, "Maximum width in the body of a struct lit before \
        falling back to vertical formatting.";
    struct_variant_width: usize, 35, true, "Maximum width in the body of a struct variant before \
        falling back to vertical formatting.";
    array_width: usize, 60, true,  "Maximum width of an array literal before falling \
        back to vertical formatting.";
    chain_width: usize, 60, true, "Maximum length of a chain to fit on a single line.";
    single_line_if_else_max_width: usize, 50, true, "Maximum line length for single line if-else \
        expressions. A value of zero means always break if-else expressions.";

    // Comments. macros, and strings
    wrap_comments: bool, false, false, "Break comments to fit on the line";
    format_code_in_doc_comments: bool, false, false, "Format the code snippet in doc comments.";
    doc_comment_code_block_width: usize, 100, false, "Maximum width for code snippets in doc \
        comments. No effect unless format_code_in_doc_comments = true";
    comment_width: usize, 80, false,
        "Maximum length of comments. No effect unless wrap_comments = true";
    normalize_comments: bool, false, false, "Convert /* */ comments to // comments where possible";
    normalize_doc_attributes: bool, false, false, "Normalize doc attributes as doc comments";
    format_strings: bool, false, false, "Format string literals where necessary";
    format_macro_matchers: bool, false, false,
        "Format the metavariable matching patterns in macros";
    format_macro_bodies: bool, true, false, "Format the bodies of macros";
    hex_literal_case: HexLiteralCase, HexLiteralCase::Preserve, false,
        "Format hexadecimal integer literals";

    // Single line expressions and items
    empty_item_single_line: bool, true, false,
        "Put empty-body functions and impls on a single line";
    struct_lit_single_line: bool, true, false,
        "Put small struct literals on a single line";
    fn_single_line: bool, false, false, "Put single-expression functions on a single line";
    where_single_line: bool, false, false, "Force where-clauses to be on a single line";

    // Imports
    imports_indent: IndentStyle, IndentStyle::Block, false, "Indent of imports";
    imports_layout: ListTactic, ListTactic::Mixed, false, "Item layout inside a import block";
    imports_granularity: ImportGranularity, ImportGranularity::Preserve, false,
        "Merge or split imports to the provided granularity";
    group_imports: GroupImportsTactic, GroupImportsTactic::Preserve, false,
        "Controls the strategy for how imports are grouped together";
    merge_imports: bool, false, false, "(deprecated: use imports_granularity instead)";

    // Ordering
    reorder_imports: bool, true, true, "Reorder import and extern crate statements alphabetically";
    reorder_modules: bool, true, true, "Reorder module statements alphabetically in group";
    reorder_impl_items: bool, false, false, "Reorder impl items";

    // Spaces around punctuation
    type_punctuation_density: TypeDensity, TypeDensity::Wide, false,
        "Determines if '+' or '=' are wrapped in spaces in the punctuation of types";
    space_before_colon: bool, false, false, "Leave a space before the colon";
    space_after_colon: bool, true, false, "Leave a space after the colon";
    spaces_around_ranges: bool, false, false, "Put spaces around the  .. and ..= range operators";
    binop_separator: SeparatorPlace, SeparatorPlace::Front, false,
        "Where to put a binary operator when a binary expression goes multiline";

    // Misc.
    remove_nested_parens: bool, true, true, "Remove nested parens";
    combine_control_expr: bool, true, false, "Combine control expressions with function calls";
    short_array_element_width_threshold: usize, 10, true,
        "Width threshold for an array element to be considered short";
    overflow_delimited_expr: bool, false, false,
        "Allow trailing bracket/brace delimited expressions to overflow";
    struct_field_align_threshold: usize, 0, false,
        "Align struct fields if their diffs fits within threshold";
    enum_discrim_align_threshold: usize, 0, false,
        "Align enum variants discrims, if their diffs fit within threshold";
    match_arm_blocks: bool, true, false, "Wrap the body of arms in blocks when it does not fit on \
        the same line with the pattern of arms";
    match_arm_leading_pipes: MatchArmLeadingPipe, MatchArmLeadingPipe::Never, true,
        "Determines whether leading pipes are emitted on match arms";
    force_multiline_blocks: bool, false, false,
        "Force multiline closure bodies and match arms to be wrapped in a block";
    fn_args_layout: Density, Density::Tall, true,
        "Control the layout of arguments in a function";
    brace_style: BraceStyle, BraceStyle::SameLineWhere, false, "Brace style for items";
    control_brace_style: ControlBraceStyle, ControlBraceStyle::AlwaysSameLine, false,
        "Brace style for control flow constructs";
    trailing_semicolon: bool, true, false,
        "Add trailing semicolon after break, continue and return";
    trailing_comma: SeparatorTactic, SeparatorTactic::Vertical, false,
        "How to handle trailing commas for lists";
    match_block_trailing_comma: bool, false, true,
        "Put a trailing comma after a block based match arm (non-block arms are not affected)";
    blank_lines_upper_bound: usize, 1, false,
        "Maximum number of blank lines which can be put between items";
    blank_lines_lower_bound: usize, 0, false,
        "Minimum number of blank lines which must be put between items";
    edition: Edition, Edition::Edition2015, true, "The edition of the parser (RFC 2052)";
    version: Version, Version::One, false, "Version of formatting rules";
    inline_attribute_width: usize, 0, false,
        "Write an item and its attribute on the same line \
        if their combined width is below a threshold";
    format_generated_files: bool, true, false, "Format generated files";

    // Options that can change the source code beyond whitespace/blocks (somewhat linty things)
    merge_derives: bool, true, true, "Merge multiple `#[derive(...)]` into a single one";
    use_try_shorthand: bool, false, true, "Replace uses of the try! macro by the ? shorthand";
    use_field_init_shorthand: bool, false, true, "Use field initialization shorthand if possible";
    force_explicit_abi: bool, true, true, "Always print the abi for extern items";
    condense_wildcard_suffixes: bool, false, false, "Replace strings of _ wildcards by a single .. \
                                                     in tuple patterns";

    // Control options (changes the operation of rustfmt, rather than the formatting)
    // [..] deleted

    // Not user-facing
    // [..] deleted
}

macro_rules! make_enum {
    ($name:ident $(, $v:ident)+) => {
        #[derive(Serialize)]
        pub enum $name {
            $( $v, )*
        }
    };
}

make_enum!(NewlineStyle, Auto, Windows, Unix, Native);
make_enum!(IndentStyle, Visual, Block);
make_enum!(Heuristics, Off, Max, Default);
make_enum!(HexLiteralCase, Preserve, Upper, Lower);
make_enum!(ListTactic, Vertical, Horizontal, HorizontalVertical, Mixed);
make_enum!(ImportGranularity, Preserve, Crate, Module, Item, One);
make_enum!(GroupImportsTactic, Preserve, StdExternalCrate, One);
make_enum!(TypeDensity, Compressed, Wide);
make_enum!(SeparatorPlace, Front, Back);
make_enum!(MatchArmLeadingPipe, Always, Never, Preserve);
make_enum!(Density, Compressed, Tall, Vertical);
make_enum!(BraceStyle, AlwaysNextLine, PreferSameLine, SameLineWhere);
make_enum!(
    ControlBraceStyle,
    AlwaysSameLine,
    ClosingNextLine,
    AlwaysNextLine
);
make_enum!(SeparatorTactic, Always, Never, Vertical);
make_enum!(Version, One, Two);

#[derive(Serialize)]
pub enum Edition {
    #[serde(rename = "2015")]
    Edition2015,
    #[serde(rename = "2018")]
    Edition2018,
    #[serde(rename = "2021")]
    Edition2021,
    #[serde(rename = "2024")]
    Edition2024,
}
