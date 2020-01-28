/* 
 * Character array implemented with in-memory char array
 */

#ifndef MEMCHARARRAY_H
#define MEMCHARARRAY_H

#include <cstddef>
#include <cstdlib>

#include "ICharArray.h"

class MemCharArray : public ICharArray {
public:
    MemCharArray();
    ~MemCharArray();
        
    char& operator[](size_t i);            
    bool allocate(size_t size);    
    bool resize(size_t size); 
    void free();
        
private:
    size_t numOfBlocks;
    char *blocks;

};

#endif /* MEMCHARARRAY_H */

