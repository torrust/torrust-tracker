-- else the randomness would be the same every run
math.randomseed(os.time())

local charset = "0123456789ABCDEF"

function hex_to_char(hex)
    local n = tonumber(hex, 16)
    local f = string.char(n)
  return f
end

function hex_string_to_char_string(hex)
    local ret = {}
    local r
    for i = 0, 19 do
        local x = i * 2
		r = hex:sub(x+1, x+2)
		local f = hex_to_char(r)
		table.insert(ret, f)
    end
    return table.concat(ret)
end

function url_encode(str)
    str = string.gsub (str, "([^0-9a-zA-Z !'()*._~-])", -- locale independent
            function (c) return string.format ("%%%02X", string.byte(c)) end)
    str = string.gsub (str, " ", "+")
    return str
end

function gen_hex_string(length)
    local ret = {}
    local r
    for i = 1, length do
        r = math.random(1, #charset)
        table.insert(ret, charset:sub(r, r))
    end
    return table.concat(ret)
end

function random_info_hash()
    local hexString = gen_hex_string(40)
    local str = hex_string_to_char_string(hexString)
    return url_encode(str)
end

function generate_unique_info_hashes(size)
    local result = {}

    for i = 1, size do
        result[i] = random_info_hash()
    end

    return result
end

info_hashes = generate_unique_info_hashes(5000000)

index = 1

-- the request function that will run at each request
request = function()
    path = "/announce?info_hash=" .. info_hashes[index] .. "&peer_id=-lt0D80-a%D4%10%19%99%A6yh%9A%E1%CD%96&port=54434&uploaded=885&downloaded=0&left=0&corrupt=0&key=A78381BD&numwant=200&compact=1&no_peer_id=1&supportcrypto=1&redundant=0"
    index = index + 1
    headers = {}
    headers["X-Forwarded-For"] = "1.1.1.1"
    return wrk.format("GET", path, headers)
end
