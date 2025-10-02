#include <stdio.h>

int add(int a, int b);
void no_return();

// gets name
char* get_name();

// Accepts a callback
// more docs
void callback_func(void (*callback)(int)); // this comment should not be included

void array_param_function(int arr[static 10]);
int variadic_func(const char *fmt, ...);
const char * const get_string(void);

void __attribute__((always_inline))
fast_func_extern(void);

// a function
int add(int a, int b) {
    return a + b;
}

static void my_static_func() {
    // do nothing
}

void no_return() {
    printf("hello");
}

// another function
char* get_name() {
    return "test";
}


/*

some preceeding comment

*/
void callback_func(void (*callback)(int)) {

}

void array_param_function(int arr[static 10]) {

}

int variadic_func(const char *fmt, ...) {

}


// 12. Const pointer to const
const char * const get_string(void) {

}

static inline void __attribute__((always_inline))
fast_func(void) {

}

void
foo1() {}

void
foo2() {

}


void
foo3() 
{

}

int main(int argc, char** argv) {
    if (argc > 1) {
        for (int i = 0; i < argc; i++) {
            printf("%s\n", argv[i]);
        }
    }
    return 0;
}
