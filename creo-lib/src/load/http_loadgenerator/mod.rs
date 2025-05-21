use crate::{handler, schema};

#[derive(Debug, Clone)]
pub enum HTTPLoadGeneratorType {
    String {
        min_length: usize,
        max_length: usize,
    },
    Integer {
        minimum: i64,
        maximum: i64,
    },
    Float {
        minimum: f64,
        maximum: f64,
    },
    Boolean {
        true_prob: f64,
    },
    Array {
        min_length: usize,
        max_length: usize,
        item: Box<HTTPLoadGeneratorType>,
    },
    Object {
        properties: Vec<(String, HTTPLoadGeneratorType)>,
    },
}

impl From<&schema::SchemaKind> for HTTPLoadGeneratorType {
    fn from(value: &schema::SchemaKind) -> Self {
        match value {
            schema::SchemaKind::Type(type_schema) => match type_schema {
                schema::Type::String(string_type) => {
                    let min = string_type.min_length.unwrap_or_default();
                    let max =
                        string_type
                            .max_length
                            .unwrap_or_else(|| if min == 0 { 10 } else { min + 1 });
                    let openapiv3::VariantOrUnknownOrEmpty::Item(_format) = string_type.format
                    else {
                        return Self::String {
                            min_length: min,
                            // Lua max is inclusive
                            max_length: max - 1,
                        };
                    };
                    // TODO: Handle formats
                    Self::String {
                        min_length: min,
                        // Lua max is inclusive
                        max_length: max - 1,
                    }
                }
                schema::Type::Number(number_type) => {
                    let mut min = number_type.minimum.unwrap_or_default();
                    // Lua min is inclusive
                    if number_type.exclusive_minimum {
                        min += 1.0;
                    }
                    let mut max = number_type.maximum.unwrap_or(min + 1.0);
                    // Lua max is inclusive
                    if number_type.exclusive_maximum {
                        max -= 1.0;
                    }

                    Self::Float {
                        minimum: min,
                        maximum: max,
                    }
                }
                schema::Type::Integer(integer_type) => {
                    let mut min = integer_type.minimum.unwrap_or_default();
                    // Lua min is inclusive
                    if integer_type.exclusive_minimum {
                        min += 1;
                    }
                    let mut max = integer_type.maximum.unwrap_or(min + 1);
                    // Lua max is inclusive
                    if integer_type.exclusive_maximum {
                        max -= 1;
                    }

                    Self::Integer {
                        minimum: min,
                        maximum: max,
                    }
                }
                schema::Type::Object(object_type) => {
                    let properties = object_type
                        .properties
                        .iter()
                        .map(|(name, schema)| (name.clone(), (&schema.schema_kind).into()))
                        .collect();
                    Self::Object { properties }
                }
                schema::Type::Array(array_type) => {
                    let min = array_type.min_items.unwrap_or_default();
                    let max =
                        array_type
                            .max_items
                            .unwrap_or_else(|| if min == 0 { 5 } else { min + 1 });
                    let item = Box::new((&array_type.items.schema_kind).into());
                    Self::Array {
                        min_length: min,
                        max_length: max,
                        item,
                    }
                }
                schema::Type::Boolean(boolean_type) => {
                    let mut count_true = 0;
                    let mut count_false = 0;
                    for el in boolean_type.enumeration.iter().flatten() {
                        if *el {
                            count_true += 1;
                        } else {
                            count_false += 1;
                        }
                    }
                    if count_true == 0 && count_false == 0 {
                        return Self::Boolean { true_prob: 0.5 };
                    }
                    Self::Boolean {
                        true_prob: (count_true as f64 / (count_false + count_true) as f64),
                    }
                }
            },
        }
    }
}

pub struct RandomFunction {
    name: &'static str,
    args: String,
}

impl HTTPLoadGeneratorType {
    fn as_random_function(&self) -> RandomFunction {
        match self {
            HTTPLoadGeneratorType::String {
                min_length,
                max_length,
            } => RandomFunction {
                name: "random_string",
                args: format!("{}, {}", min_length, max_length),
            },
            HTTPLoadGeneratorType::Integer { minimum, maximum } => RandomFunction {
                name: "random_int",
                args: format!("{}, {}", minimum, maximum),
            },
            HTTPLoadGeneratorType::Float { minimum, maximum } => RandomFunction {
                name: "random_float",
                args: format!("{}, {}", minimum, maximum),
            },
            HTTPLoadGeneratorType::Boolean { true_prob } => RandomFunction {
                name: "random_bool",
                args: true_prob.to_string(),
            },
            HTTPLoadGeneratorType::Array {
                min_length,
                max_length,
                item,
            } => {
                let item_func = item.as_random_function();
                RandomFunction {
                    name: "random_array",
                    args: format!(
                        "{}, {}, {}, {}",
                        min_length, max_length, item_func.name, item_func.args
                    ),
                }
            }
            HTTPLoadGeneratorType::Object { properties } => {
                let mut args = Vec::with_capacity(properties.len());
                for (name, prop) in properties {
                    let prop_func = prop.as_random_function();
                    args.push(format!(
                        r#"{{ name = "{}", func = {}, args = {{ {} }} }}"#,
                        name, prop_func.name, prop_func.args
                    ));
                }
                RandomFunction {
                    name: "random_object",
                    args: args.join(", "),
                }
            }
        }
    }

    pub fn as_function_call(&self) -> String {
        let func = self.as_random_function();
        format!("{}({})", func.name, func.args)
    }
}

#[derive(Debug, Clone)]
pub struct RequestData {
    pub query: Vec<QueryComponent>,
    pub body: Option<HTTPLoadGeneratorType>,
}

#[derive(Debug, Clone)]
pub struct QueryComponent {
    pub name: String,
    pub value: HTTPLoadGeneratorType,
}

impl QueryComponent {
    pub fn as_lua_source(&self) -> String {
        let value_src = self.value.as_function_call();
        format!(r#""{}="..{}"#, self.name, value_src)
    }
}

impl From<&handler::Function> for RequestData {
    fn from(value: &handler::Function) -> Self {
        let mut query = Vec::with_capacity(value.signature.parameters.len());
        let mut body = Option::None;

        for param in &value.signature.parameters {
            if param.schema.get_object_schema().is_some()
                || param.schema.get_array_schema_type().is_some()
            {
                body = Some(HTTPLoadGeneratorType::from(&param.schema.schema_kind))
            } else {
                query.push(QueryComponent {
                    name: param.as_name(),
                    value: (&param.schema.schema_kind).into(),
                });
            }
        }

        Self { query, body }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub endpoint_id: usize,
    pub service_id: usize,
    pub path: String,
    pub data: RequestData,
}

impl Request {
    pub fn as_lua_source(&self) -> String {
        let mut src = String::new();
        src.push_str("function ");
        src.push_str(self.as_lua_function().as_str());
        src.push_str("()\n  return");
        if self.data.body.is_some() {
            src.push_str(r#" "[POST]".."#);
        }
        let query = self
            .data
            .query
            .iter()
            .map(QueryComponent::as_lua_source)
            .collect::<Vec<_>>()
            .join(r#".."&".."#);
        src.push_str(format!(r#" services[{}].."{}""#, self.service_id, self.path).as_str());
        if !query.is_empty() {
            src.push_str(format!(r#".."?"..{}"#, query).as_str());
        }
        if let Some(body) = &self.data.body {
            src.push_str(
                format!(r#".."[JSON]"..json.encode({})"#, body.as_function_call()).as_str(),
            );
        }

        src.push_str("\nend\n\n");

        src
    }

    pub fn as_lua_function(&self) -> String {
        format!("endpoint_{}_request_data", self.endpoint_id)
    }
}

#[derive(Debug, Clone)]
pub struct Service {
    pub id: usize,
    pub url: String,
}

impl Service {
    pub fn as_lua_table_entry(&self) -> String {
        format!(r#"[{}] = "{}""#, self.id, self.url)
    }
}

pub struct Script {
    pub services: Vec<Service>,
    pub requests: Vec<Request>,
}

impl Script {
    pub fn as_lua_source(&self) -> String {
        // OnCycle Function Source
        let mut src = String::new();
        src.push_str(
            r#"--[[
  Gets called at the beginning of each "call cycle", perform as much work as possible here.
  Initialize all global variables here.
  Note that math.random is already initialized using a fixed seed (5) for reproducibility.
--]]
function onCycle()
"#,
        );

        src.push_str("  services = {\n");
        src.push_str(
            format!(
                "    {}",
                self.services
                    .iter()
                    .map(Service::as_lua_table_entry)
                    .collect::<Vec<_>>()
                    .join(",\n    ")
            )
            .as_str(),
        );
        src.push_str("\n  }\nend\n\n");

        // OnCall Function Source

        src.push_str(r#"
--[[
  Gets called with ever increasing callnums for each http call until it returns nil.
  Once it returns nil, onCycle() is called again and callnum is reset to 1 (Lua convention).

  Here, you can use our HTML helper functions for conditional calls on returned texts (usually HTML, thus the name).
  We offer:
  - html.getMatches( regex )
    Returns all lines in the returned text stream that match a provided regex.
  - html.extractMatches( prefixRegex, postfixRegex )
    Returns all matches that are preceeded by a prefixRegex match and followed by a postfixRegex match.
    The regexes must one unique match for each line in which they apply.
  - html.extractMatches( prefixRegex, matchingRegex, postfixRegex )
    Variant of extractMatches with a matching regex defining the string that is to be extracted.
--]]
function onCall(callnum)
"#);

        for (idx, request) in self.requests.iter().enumerate() {
            let keyword = if idx == 0 { "if" } else { "elseif" };
            src.push_str(
                format!(
                    "  {} callnum == {} then
    return {}()
",
                    keyword,
                    idx + 1,
                    request.as_lua_function()
                )
                .as_str(),
            );
        }
        src.push_str(
            "  else
    return nil
  end
end\n\n",
        );

        for request in &self.requests {
            src.push_str(request.as_lua_source().as_str());
        }

        src.push_str(LUA_JSON_ENCODE);
        src.push_str(RANDOM_FUNCTIONS);

        src
    }
}

const LUA_JSON_ENCODE: &str = r#"
--
-- json.lua
--
-- Copyright (c) 2020 rxi
--
-- Permission is hereby granted, free of charge, to any person obtaining a copy of
-- this software and associated documentation files (the "Software"), to deal in
-- the Software without restriction, including without limitation the rights to
-- use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
-- of the Software, and to permit persons to whom the Software is furnished to do
-- so, subject to the following conditions:
--
-- The above copyright notice and this permission notice shall be included in all
-- copies or substantial portions of the Software.
--
-- THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
-- IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
-- FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
-- AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
-- LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
-- OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
-- SOFTWARE.
--

json = { _version = "0.1.2" }

-------------------------------------------------------------------------------
-- Encode
-------------------------------------------------------------------------------

local encode

local escape_char_map = {
	["\\"] = "\\",
	['"'] = '"',
	["\b"] = "b",
	["\f"] = "f",
	["\n"] = "n",
	["\r"] = "r",
	["\t"] = "t",
}

local escape_char_map_inv = { ["/"] = "/" }
for k, v in pairs(escape_char_map) do
	escape_char_map_inv[v] = k
end

local function escape_char(c)
	return "\\" .. (escape_char_map[c] or string.format("u%04x", c:byte()))
end

local function encode_nil(val)
	return "null"
end

local function encode_table(val, stack)
	local res = {}
	stack = stack or {}

	-- Circular reference?
	if stack[val] then
		error("circular reference")
	end

	stack[val] = true

	if rawget(val, 1) ~= nil or next(val) == nil then
		-- Treat as array -- check keys are valid and it is not sparse
		local n = 0
		for k in pairs(val) do
			if type(k) ~= "number" then
				error("invalid table: mixed or invalid key types")
			end
			n = n + 1
		end
		if n ~= #val then
			error("invalid table: sparse array")
		end
		-- Encode
		for i, v in ipairs(val) do
			table.insert(res, encode(v, stack))
		end
		stack[val] = nil
		return "[" .. table.concat(res, ",") .. "]"
	else
		-- Treat as an object
		for k, v in pairs(val) do
			if type(k) ~= "string" then
				error("invalid table: mixed or invalid key types")
			end
			table.insert(res, encode(k, stack) .. ":" .. encode(v, stack))
		end
		stack[val] = nil
		return "{" .. table.concat(res, ",") .. "}"
	end
end

local function encode_string(val)
	return '"' .. val:gsub('[%z\1-\31\\"]', escape_char) .. '"'
end

local function encode_number(val)
	-- Check for NaN, -inf and inf
	if val ~= val or val <= -math.huge or val >= math.huge then
		error("unexpected number value '" .. tostring(val) .. "'")
	end
	return string.format("%.14g", val)
end

local type_func_map = {
	["nil"] = encode_nil,
	["table"] = encode_table,
	["string"] = encode_string,
	["number"] = encode_number,
	["boolean"] = tostring,
}

encode = function(val, stack)
	local t = type(val)
	local f = type_func_map[t]
	if f then
		return f(val, stack)
	end
	error("unexpected type '" .. t .. "'")
end

function json.encode(val)
	return (encode(val))
end
"#;

const RANDOM_FUNCTIONS: &str = r#"
--- Generate a random string.
--- @param min_length integer The minimum length of the string (inclusive)
--- @param max_length integer The maximum length of the string (inclusive)
--- @return string # The random string
function random_string(min_length, max_length)
	local maxLength = max_length or min_length
	local length = math.random(min_length, maxLength)
	local str = ""
	for i = 1, length do
		local c = string.char(math.random(65, 65 + 25))
		if math.random() > 0.5 then
			c = c:lower()
		end
		str = str .. c
	end
	return str
end

---Generates a random floating point number.
---@param lower number The lower bound of the random float (inclusive)
---@param upper number The upper bound of the random float (inclusive)
---@return number # The random float
function random_float(lower, upper)
	return lower + math.random() * (upper - lower)
end

---Generates a random integer.
---@param lower integer The lower bound of the random integer (inclusive)
---@param upper integer The upper bound of the random integer (inclusive)
---@return integer # The random integer
function random_int(lower, upper)
	return math.random(lower, upper)
end

---Generates a random boolean value.
---@param true_prob number The probability of 'true'
---@return boolean # The random boolean
function random_bool(true_prob)
	return math.random() < true_prob
end

---Generates a random array.
---@param min_length integer The minimum number of elements in the array
---@param max_length integer The maximum number of elements in the array
---@param item function The function to generate the random array elements
---@param ... any The function arguments to pass to the `item` function
---@return table # The random array
function random_array(min_length, max_length, item, ...)
	local length = math.random(min_length, max_length)
	local arr = {}
	for i = 1, length do
		arr[i] = item(...)
	end
	return arr
end

---Generates a random object.
---@param ... { name: string, func: function, args: any} Table specifying the `name` of the object property and its generator function and arguments.
---@return table # The random object.
function random_object(...)
	local args = { ... }
	local out = {}
	for i = 1, #args do
		local prop = args[i]
		out[prop["name"]] = prop["func"](table.unpack(prop["args"]))
	end
	return out
end
"#;
