#include <stdlib.h>
#include <stdio.h>

typedef struct Node {
    unsigned i;
    struct Node* next;
} Node;

int main (int argc, char** argv)
{
    const int skip = 303;
    const int size = 50000000;

    Node* head = calloc(sizeof(Node), 1);
    head->next = head;

    for (unsigned i=1; i <= size; ++i)
    {
        if ((i % 50000) == 0)
        {
            printf("%f\n", i / (float)size);
        }

        for (unsigned j=0; j < skip; ++j)
        {
            head = head->next;
        }

        Node* next = head->next;
        head->next = malloc(sizeof(Node));
        *(head->next) = (Node){ .i=i, .next=next };
        head = head->next;
    }

    for (unsigned i=0; i < size; ++i)
    {
        if (head->i == 0)
        {
            printf("%i\n", head->next->i);
        }
        head = head->next;
    }
    return 0;
}
