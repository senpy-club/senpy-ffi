use v5.30.0;
use FFI::Raw;

my $status = FFI::Raw->new(
  "target/debug/libsenpy_ffi.so",
  "status",
  FFI::Raw::int
);

my $c_status = $status->call();
my $senpy_status;

if ($c_status == 1) {
  $senpy_status = "up";
} elsif ($c_status == 0) {
  $senpy_status = "down";
} elsif ($c_status == -1) {
  $senpy_status = "not down, but unreachable";
} else {
  $senpy_status = "unknown";
}

say "status: api.senpy.club is " . $senpy_status;
