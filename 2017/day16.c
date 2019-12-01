#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

const int DANCE_SIZE = 'p' - 'a' + 1;
struct cmd {
    bool partner;
    unsigned remap[DANCE_SIZE];
};
const char* input_file = "input16.txt";
const uint64_t reps = 1000000000;

uint64_t to_int(uint8_t* dance)
{
    uint64_t out = 0;
    for (unsigned i=0; i < DANCE_SIZE; ++i)
    {
        out |= ((uint64_t)((dance[i] - 'a') & 15)) << (4 * i);
    }
    return out;
}
struct seen { uint64_t a, b; };

void from_int(uint64_t id, uint8_t* dance)
{
    for (unsigned i=0; i < DANCE_SIZE; ++i)
    {
        dance[i] = ((id >> (4 * i)) & 15) + 'a';
    }
}

int main(int argc, char** argv)
{
    uint64_t cmd_count = 0;
    struct cmd* tape = malloc(sizeof(struct cmd));;

    {
        uint64_t tape_length = 1;

        FILE* input = fopen(input_file, "r");
        if (!input) return -1;

        char c;
        while ((c = fgetc(input)) != EOF)
        {
            if (cmd_count == tape_length)
            {
                tape_length *= 2;
                tape = realloc(tape, sizeof(struct cmd) * tape_length);
            }

            if (c == 's')
            {
                int spin_count;
                fscanf(input, "%i", &spin_count);
                for (unsigned i=0; i < spin_count; ++i)
                {
                    tape[cmd_count].remap[i] = DANCE_SIZE - spin_count + i;
                }
                for (unsigned i=spin_count; i < DANCE_SIZE; ++i)
                {
                    tape[cmd_count].remap[i] = i - spin_count;
                }
                tape[cmd_count].partner = false;
            }
            else if (c == 'x')
            {
                int a, b;
                fscanf(input, "%i/%i", &a, &b);
                for (unsigned i=0; i < DANCE_SIZE; ++i)
                {
                    if (i == a)
                    {
                        tape[cmd_count].remap[i] = b;
                    }
                    else if (i == b)
                    {
                        tape[cmd_count].remap[i] = a;
                    }
                    else
                    {
                        tape[cmd_count].remap[i] = i;
                    }
                }
                tape[cmd_count].partner = false;
            }
            else if (c == 'p')
            {
                tape[cmd_count].partner = true;
                char a, b;
                fscanf(input, "%c/%c", &a, &b);
                tape[cmd_count].remap[0] = a;
                tape[cmd_count].remap[1] = b;
            }
            else
            {
                return -1;
            }
            cmd_count++;
            fgetc(input);
        }
        fclose(input);
    }

    {   // Collapse remap commands into each other
        int index = -1;
        bool was_remap = false;
        for (unsigned i=0; i < cmd_count; ++i)
        {
            if (i != 0 && !tape[index].partner && !tape[i].partner)
            {
                unsigned remap[DANCE_SIZE];
                memcpy(remap, tape[index].remap, sizeof(remap));
                for (unsigned j=0; j < DANCE_SIZE; ++j)
                {
                    tape[index].remap[j] = remap[tape[i].remap[j]];
                }
            }
            else
            {
                index++;
                memcpy(&tape[index], &tape[i], sizeof(struct cmd));
            }
        }
        printf("went from %i to %i commands\n", cmd_count, index + 1);
        cmd_count = index + 1;
    }

    {
        // Double-buffering for dancing
        uint8_t buf[DANCE_SIZE * 2];
        uint8_t* dance = buf;
        uint8_t* dance_ = buf + DANCE_SIZE;

        for (unsigned i=0; i < DANCE_SIZE; ++i)
        {
            dance[i] = 'a' + i;
        }

        bool found[256] = {false};

        struct seen* seen_list = malloc(sizeof(struct seen));
        uint64_t seen_list_size = 1;
        uint64_t seen_list_count = 0;

        for (uint64_t q=0; q < reps; ++q)
        {
            if ((q % 1000000) == 0)
            {
                printf("%i (%i)\n", q / 1000000, seen_list_count);
            }
            uint64_t id = to_int(dance);
            from_int(id, dance_);
            bool found = false;
            for (unsigned i=0; i < seen_list_count; ++i)
            {
                if (id == seen_list[i].a)
                {
                    found = true;
                    from_int(seen_list[i].b, dance);
                    break;
                }
            }

            if (!found)
            {
                for (unsigned i=0; i < cmd_count && !found; ++i)
                {
                    // Special case for the partner operation, which requires search
                    if (tape[i].partner)
                    {
                        unsigned found = 0;
                        for (unsigned j=0; j < DANCE_SIZE && found < 2; ++j)
                        {
                            if (dance[j] == tape[i].remap[0])
                            {
                                dance[j] = tape[i].remap[1];
                                found++;
                            }
                            else if (dance[j] == tape[i].remap[1])
                            {
                                dance[j] = tape[i].remap[0];
                                found++;
                            }
                        }
                    }
                    // Otherwise, blindly execute the remap
                    else
                    {
                        for (unsigned j=0; j < DANCE_SIZE; ++j)
                        {
                            dance_[j] = dance[tape[i].remap[j]];
                        }
                        uint8_t* tmp = dance;
                        dance = dance_;
                        dance_ = tmp;
                    }
                }

                uint64_t id_after = to_int(dance);
                if (seen_list_size == seen_list_count)
                {
                    seen_list_size *= 2;
                    seen_list = realloc(seen_list, sizeof(struct seen) * seen_list_size);
                }

                seen_list[seen_list_count++] = (struct seen){id, id_after};
            }
        }
        for (unsigned i=0; i < DANCE_SIZE; ++i)
        {
            printf("%c", dance[i]);
        }
        printf("\n");
    }
}
