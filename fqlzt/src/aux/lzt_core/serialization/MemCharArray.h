/* 
 * Character array implemented with in-memory char array
 */

#ifndef MEMCHARARRAY_H
#define MEMCHARARRAY_H

#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <fstream>

#include "ICharArray.h"
#include "util/filesystem_utils.h"
#include "serialization_legacy/SerializationUtils.h"

using namespace std;

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
    void writeToStream(ostream& stream);
    void readFromStream(istream& stream);
        
private:
    size_t numOfBlocks;
    char *blocks;
        
    static const string PERSIST_FNAME;
};

inline char& MemCharArray::operator[](size_t i) {    
    return blocks[i];
}

#endif /* MEMCHARARRAY_H */

