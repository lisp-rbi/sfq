/* 
 * Character array implemented with in-memory char array
 */

#ifndef MEMCHARARRAY_H
#define MEMCHARARRAY_H

#include <cstddef>

#include "ICharArray.h"

class MemCharArray : public ICharArray {
public:
    MemCharArray();
    // ? MemCharArray(const MemCharArray& orig);
    char& operator[](size_t i) = 0;
    ~MemCharArray();
private:

};

#endif /* MEMCHARARRAY_H */

