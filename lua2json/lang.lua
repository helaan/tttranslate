-- NOTE: as we are not running inside Gmod, we need to supply our own Lua library
-- I picked https://github.com/rxi/json.lua
json = require "json"

require "ordered"

LANG = {}
local languages = {}

function LANG.CreateLanguage(name)
	languages[name] = Ordered()
	return languages[name]
end

-- Evaluate all input files
for argc, argv in ipairs(arg) do
	print("Reading " .. argv)
	dofile(argv)
end

-- Process all created languages
for langname,lang in pairs(languages) do
	print("Saving " .. langname .. ".json ...")

	--print(show(lang))
	file = io.open(langname .. ".json", "w")
	file:write(json.encode(lang))
	file:close()
end
