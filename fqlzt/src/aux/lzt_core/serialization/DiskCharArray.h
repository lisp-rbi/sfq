/* 
 * Character array implemented on disk.
 */

#ifndef DISKCHARARRAY_H
#define DISKCHARARRAY_H

#include <cstddef>
#include <cstdlib>
#include <cstdio>
#include <string>
#include <iostream>
#include <unistd.h>

#include "ICharArray.h"
#include "util/utils.h"

class DiskArrayChar;

class DiskCharArray {
public:
    DiskCharArray();
    ~DiskCharArray();
        
    DiskArrayChar operator[](size_t i);            
    bool allocate(size_t size);    
    bool resize(size_t size); 
    void freeMemory();
    
    friend class DiskArrayChar;
        
private:
    size_t numOfBlocks;    
    FILE *file;
    char *iobuffer; // handled by openFile/closeFile methods
    string fname;
    bool fileOpened;    
    
    static const bool DEBUG = false;
    static const size_t BUFFER_SIZE = 1024; // if set to 0, no buffering is used
    
    bool closeFile();
    bool openFile();
    bool flushFile();
    bool deleteFile();
    bool fileExists();
    bool createEmptyFile();
    void bindToRandomFile();        
    void writeCharacter(size_t index, char ch);
    char readCharacter(size_t index);

};

/** char-like class that is returned by [] operator of DiskCharArray. */
class DiskArrayChar {
public:
    DiskArrayChar(DiskCharArray* arr, size_t idx, char c): ch(c), index(idx), array(arr) {};
    ~DiskArrayChar() {};
        
    operator char() { return ch; }    
    DiskArrayChar& operator |=(char c);  
    DiskArrayChar& operator &=(char c);

private:
    DiskCharArray* array;
    size_t index;
    char ch;    
    
};

inline DiskArrayChar& DiskArrayChar::operator |=(char c) { 
    ch |= c; 
    array->writeCharacter(index, ch);
    return *this; 
}

inline DiskArrayChar& DiskArrayChar::operator &=(char c) {  
    ch &= c; 
    array->writeCharacter(index, ch);
    return *this;    
}

#endif /* DISKCHARARRAY_H */

