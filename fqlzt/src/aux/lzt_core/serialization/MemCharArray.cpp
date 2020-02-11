#include "MemCharArray.h"

MemCharArray::MemCharArray(): numOfBlocks(0), blocks(NULL) { }

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

void MemCharArray::freeMemory() {
    if (blocks != NULL) {        
        free(blocks);
        blocks = NULL;
    }
}

bool MemCharArray::persist(string f) {
    return true;
}

bool MemCharArray::load(string f) {
    return true;
}

MemCharArray::~MemCharArray() { }

