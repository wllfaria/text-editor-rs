pub static DEFAULT_LIGHT: &str = r##"name = "glyph-default-light.toml"
[appearance]
bg = "#101010"
fg = "#cecece"

[float]
bg = "#242424"
fg = "#cecece"

[gutter]
bg = "#242424"
fg = "#666666"

[statusline]
file_name = { fg = "#909090", bg = "#242424" }
mode = { fg = "#CECECE", bg = "#C586C0", bold = true }
cursor = { fg = "#4EC9B0", bg = "#242424" }
appearance = { bg = "#242424" }

[tokens]
function = { fg = "#7daea3" }
"function.method" = { fg = "#82aaff" }
"function.macro" = { fg = "#ff9e64" }
"constant.builtin" = { fg = "#ffcc66" }
constant = { fg = "#d8a657" }
type = { fg = "#569CD6" }
"type.builtin" = { fg = "#4EC9B0" }
constructor = { fg = "#B5CEA8" }
property = { fg = "#CE9178" }
"variable.parameter" = { fg = "#9CDCFE" }
"variable.builtin" = { fg = "#C586C0" }
label = { fg = "#D7BA7D" }
comment = { fg = "#608B4E" }
"punctuation.bracket" = { fg = "#D4D4D4" }
"punctuation.delimiter" = { fg = "#D4D4D4" }
keyword = { fg = "#C586C0" }
string = { fg = "#CE9178" }
escape = { fg = "#d7ba7d" }
operator = { fg = "#569CD6" }
attribute = { fg = "#4EC9B0" }
"##;
