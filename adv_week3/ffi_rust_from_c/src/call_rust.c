#include <stdio.h>

extern int add(int a, int b);

int main() {
    int x = add(1, 2);
    printf("x: %d\n\r", x);

    return 0;
}