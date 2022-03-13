local ffi = require("ffi")

local os = ffi.os
local extension

if os == "Linux" then
  extension = "so"
elseif os == "Windows" then
  extension = "dll"
else
  extension = "dylib"
end

ffi.cdef[[
  int status(void);
]]

local C = ffi.load("target/debug/senpy_ffi." .. extension)

-- status
local c_status = C.status()
local status

if c_status == 1 then
  status = "up"
elseif c_status == 0 then
  status = "down"
elseif c_status == -1 then
  status = "not down, but unreachable"
else
  status = "unknown"
end

print("status: api.senpy.club is " .. status)
