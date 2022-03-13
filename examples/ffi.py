import sys
from cffi import FFI

prefix: str = {"win32": ""}.get(sys.platform, "lib")
extension: str = {"darwin": ".dylib", "win32": ".dll"}.get(sys.platform, ".so")

ffi = FFI()
ffi.cdef(
  """
  struct Random { char *language; char *image; };
  
  char **language(const char *);
  
  char **languages(void);
  
  struct Random *random_new(void);
  void random_populate(struct Random *);
  void random_free(struct Random *);
  char *random_get(const struct Random *, const char *);
  
  int status(void);
  """
)

C = ffi.dlopen("target/debug/{}senpy_ffi{}".format(prefix, extension))

# languages
languages = C.languages()
languages_list: list[str] = []

for i in range(int(ffi.string(languages[0]))):
  languages_list.append(ffi.string(languages[i]).decode("utf-8"))

languages_list.pop(0)

print("languages:", languages_list)

# random
class Random:
  def __init__(self) -> None:
    self.obj = C.random_new()

  def __enter__(self):
    return self

  def __exit__(self, _, __, ___) -> None:
    C.random_free(self.obj)

  def populate(self) -> None:
    C.random_populate(self.obj)

  def get(self) -> (str, str):
    return (
      ffi.string(C.random_get(self.obj, "language".encode("utf-8"))).decode("utf-8"),
      ffi.string(C.random_get(self.obj, "image".encode("utf-8"))).decode("utf-8"),
    )

with Random() as random:
  random.populate()
  print("random:", random.get())

# status
c_status: int = C.status()
status: str = ""

if c_status == 1:
  status = "up"
elif c_status == 0:
  status = "down"
elif c_status == -1:
  status = "not down, but unreachable"
else:
  status = "unknown"

print("status: api.senpy.club is", status)

# language
images = C.language("ASM".encode("utf-8"))
images_list: list[str] = []

for i in range(int(ffi.string(images[0]))):
  images_list.append(ffi.string(images[i]).decode("utf-8"))

images_list.pop(0)

print("images:", images_list)
