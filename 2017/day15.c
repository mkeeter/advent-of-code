#include <stdint.h>
#include <stdio.h>

int main(int argc, char** argv)
{
    int64_t a = 116;
    int64_t b = 299;

    unsigned count = 0;

    for (unsigned i=0; i < 40000000; ++i)
    {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        count += (a & 65535) == (b & 65535);
    }
    printf("count: %i\n", count);
}
