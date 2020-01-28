#include "MemCharArray.h"

MemCharArray::MemCharArray(): numOfBlocks(0), blocks(NULL) { }

char& MemCharArray::operator[](size_t i) {
    char a = 'a';
    return a;
}

bool MemCharArray::allocate(size_t size) {
    blocks = (char *)malloc(size);    
    return blocks != NULL;
}

bool MemCharArray::resize(size_t size) {
    void *newblocks = realloc(blocks, size);
    if (newblocks != NULL) {
        blocks = (char *)newblocks;
        return true;
    }
    else return false;    
}

void MemCharArray::free() {
    if (blocks != NULL) {
        free(blocks);
        blocks = NULL;
    }
}

MemCharArray::~MemCharArray() { }

