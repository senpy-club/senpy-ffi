<?php

$extension = (PHP_OS_FAMILY == "Darwin" ? "dylib" : "so");

$ffi = FFI::cdef(
  "int status(void);",
  "target/debug/libsenpy_ffi.$extension"
);

$c_status = $ffi->status();
$status;

if ($c_status == 1):
  $status = "up";
elseif ($c_status == 0):
  $status = "down";
elseif ($c_status == -1):
  $status = "not down, but unreachable";
else:
  $status = "unknown";
endif;

echo "status: api.senpy.club is " . $status . "\r\n";

?>
