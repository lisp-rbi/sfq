/* 
 * Character array implemented with in-memory char array
 */

#ifndef MEMCHARARRAY_H
#define MEMCHARARRAY_H

#include <cstddef>
#include <cstdlib>

#include "ICharArray.h"

class MemCharArray : public ICharArray<char&> {
public:
    MemCharArray();
    ~MemCharArray();
        
    char& operator[](size_t i);            
    bool allocate(size_t size);    
    bool resize(size_t size); 
    void freeMemory();
    bool persist(string f);
    bool load(string f);
        
private:
    size_t numOfBlocks;
    char *blocks;

};

inline char& MemCharArray::operator[](size_t i) {    
    return blocks[i];
}

#endif /* MEMCHARARRAY_H */

