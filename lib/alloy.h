#include <stdint.h>
#include "cdata/arrow/c/abi.h"

void init_rs_logging();
void test_rs_logging(char* message);

unsigned load_from_chunks(
    const struct ArrowArray *arr_ptr,
    const struct ArrowSchema *arr_sch,
    uintptr_t n_chunks,
);
