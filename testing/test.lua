require "math"
require "os"

math.randomseed(os.time())
math.random(); math.random(); math.random();

local curl = Curl(
	"get",
	"https://cat-fact.herokuapp.com/facts/random?animal_type=cat,horse,lizard,bear,rabbit&amount=" .. math.random(10, 50),
	{},
	""
);

local response = curl:run();

local content_type = response:get_content_type();

if content_type:find('^' .. "text/") ~= nil --[[ text mime types ]] then
	set_response(response:text())
else --[[ json mime type ]]

	local json = response:json();

	-- Add title to dom
	append_response("\tAnimal Facts:\n\n\n")
	
	-- loop through api response & append animal facts to dom
	for _, fact in ipairs(json) do
		append_response(fact["type"].." Fact:\n"..fact["createdAt"].."\n\t"..fact["text"].."\n\n")
	end

end