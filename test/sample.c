#include <stdlib.h>
#include <string.h>
__attribute__((noinline)) void foo(void) {
  int x = 42;
  x += 24;
}
int main(int argc, char **argv) {
  if (argc < 2)
    exit(1);
  char *s = argv[1];
  size_t l = strlen(s);
  if (l > 0 && s[0] == 'b')
    if (l > 1 && s[1] == 'u')
      if (l > 2 && s[2] == 'g')
        abort();
  if (l > 1 && s[1] == 'x')
    foo();
}
