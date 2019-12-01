#include <stdint.h>
#include <stdio.h>

int main(int argc, char** argv)
{
    int64_t a = 116;
    int64_t b = 299;

    unsigned count = 0;

    for (unsigned i=0; i < 5000000; ++i)
    {
        do
            a = (a * 16807) % 2147483647;
        while (a % 4);

        do
            b = (b * 48271) % 2147483647;
        while (b % 8);

        count += (a & 65535) == (b & 65535);
    }
    printf("Got count %i\n", count);
}
