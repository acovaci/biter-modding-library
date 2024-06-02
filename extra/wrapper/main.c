/*
 * This is a very simple DLL that calls the Rust constructor and destructor functions. We need this
 * because the Rust compiler doesn't support the `constructor` and `destructor` attributes.
 *
 * Basically, these two functions are called when the DLL is loaded and unloaded, respectively.
 */

#include <stdio.h>

void rust_constructor();
void rust_destructor();

__attribute__((constructor))
void my_constructor() {
    rust_constructor();
}

__attribute__((destructor))
void my_destructor() {
    rust_destructor();
}
