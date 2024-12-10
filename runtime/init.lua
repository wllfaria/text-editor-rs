--- @class glyph
--- @field u glyph.utils
--- @field api glyph.api
--- @field options glyph.options
--- @field _core glyph.core
--- @field _startup function

--- @class glyph.core
--- @field set_keymap_command fun(mode: "n" | "i" | "c" | "v", keys: string, command: string, opts?: KeymapOpts)
--- @field set_keymap_function fun(mode: "n" | "i" | "c" | "v", keys: string, command: function, opts?: KeymapOpts)
--- @field set_hl_group fun(name: string, opts: glyph.colors.hl_group)
--- @field get_editor_mode fun(): string

--- @class glyph.options
--- @field cursor glyph.options.cursor
--- @field gutter glyph.options.gutter
--- @field statusline glyph.options.statusline

--- @class glyph.options.cursor
--- @field style "block" | "steady_bar"

--- @class glyph.options.gutter
--- @field enabled boolean
--- @field anchor "left" | "right"
--- @field line_numbers "absolute" | "relative" | "relative_numbered"
--- @field sign_column "all" | "git" | "lsp"

--- @class glyph.options.statusline
--- @field left glyph.options.statusline.section[]
--- @field right glyph.options.statusline.section[]

--- @class glyph.options.statusline.section
--- @field content string|fun(): string
--- @field style string|glyph.colors.hl_group

--- @type glyph
local glyph = require("glyph")

glyph.u = require("utils")
glyph.api = require("api")

glyph._startup = function()
  require("defaults.statusline")
end

require("defaults")
