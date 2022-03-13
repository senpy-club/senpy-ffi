c_status = ccall((:status, "target/debug/senpy_ffi"), Int32, ())
status = ""

if c_status == 1
  status = "up"
elseif c_status == 0
  status = "down"
elseif c_status == -1
  status = "not down, but unreachable"
else
  status = "unknown"
end

print("status: api.senpy.club is ", status)
