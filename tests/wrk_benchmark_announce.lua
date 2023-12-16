function generate_unique_info_hashes(size)
    local result = {}
    local seen = {}

    for i = 0, size - 1 do
        local bytes = {}
        bytes[1] = i & 0xFF
        bytes[2] = (i >> 8) & 0xFF
        bytes[3] = (i >> 16) & 0xFF
        bytes[4] = (i >> 24) & 0xFF

        local info_hash = bytes
        local key = table.concat(info_hash, ",")

        if not seen[key] then
            table.insert(result, info_hash)
            seen[key] = true
        end
    end

    return result
end

info_hashes = generate_unique_info_hashes(10000000)

index = 0

-- the request function that will run at each request
request = function()
    path = "/announce?info_hash=" .. info_hashes[index] .. "&peer_id=-lt0D80-a%D4%10%19%99%A6yh%9A%E1%CD%96&port=54434&uploaded=885&downloaded=0&left=0&corrupt=0&key=A78381BD&numwant=200&compact=1&no_peer_id=1&supportcrypto=1&redundant=0"
    index += 1
    headers = {}
    headers["X-Forwarded-For"] = "1.1.1.1"
    return wrk.format("GET", path, headers)
end
