#include "luadns.h"

int main()
{
  printf("This is a triumph...\n");
  return luadns_start("init.lua");
}