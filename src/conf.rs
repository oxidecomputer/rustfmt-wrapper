// Copyright 2022 Oxide Computer Company

pub mod config {
    use serde::Serialize;

    /// `rustfmt` configuration.
    ///
    /// See the [`rustfmt` documentation](https://rust-lang.github.io/rustfmt)
    /// for the descriptions of these stable and non-stable options.
    #[derive(Serialize, Default)]
    pub struct Config {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hard_tabs: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tab_spaces: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub newline_style: Option<NewlineStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub indent_style: Option<IndentStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_small_heuristics: Option<Heuristics>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fn_call_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attr_fn_like_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub struct_lit_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub struct_variant_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub array_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chain_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub single_line_if_else_max_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub wrap_comments: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_code_in_doc_comments: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub normalize_comments: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub normalize_doc_attributes: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_strings: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_macro_matchers: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_macro_bodies: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hex_literal_case: Option<HexLiteralCase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub empty_item_single_line: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub struct_lit_single_line: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fn_single_line: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub where_single_line: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub imports_indent: Option<IndentStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub imports_layout: Option<ListTactic>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub imports_granularity: Option<ImportGranularity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group_imports: Option<GroupImportsTactic>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub merge_imports: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reorder_imports: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reorder_modules: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reorder_impl_items: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_punctuation_density: Option<TypeDensity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub space_before_colon: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub space_after_colon: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub spaces_around_ranges: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub binop_separator: Option<SeparatorPlace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub remove_nested_parens: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub combine_control_expr: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub short_array_element_width_threshold: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub overflow_delimited_expr: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub struct_field_align_threshold: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enum_discrim_align_threshold: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub match_arm_blocks: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub match_arm_leading_pipes: Option<MatchArmLeadingPipe>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub force_multiline_blocks: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fn_args_layout: Option<Density>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub brace_style: Option<BraceStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub control_brace_style: Option<ControlBraceStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trailing_semicolon: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trailing_comma: Option<SeparatorTactic>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub match_block_trailing_comma: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blank_lines_upper_bound: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blank_lines_lower_bound: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub edition: Option<Edition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<Version>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inline_attribute_width: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_generated_files: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub merge_derives: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_try_shorthand: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_field_init_shorthand: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub force_explicit_abi: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub condense_wildcard_suffixes: Option<bool>,
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
}
