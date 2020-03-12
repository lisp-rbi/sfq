/*
 * Character array stored on disk.
 */

#ifndef DISKCHARARRAY_H
#define DISKCHARARRAY_H

#include <cstddef>
#include <cstdlib>
#include <cstdio>
#include <string>
#include <iostream>
#include <unistd.h>
#include <assert.h>

#include "ICharArray.h"
#include "../serialization_legacy/SerializationUtils.h"
#include "../util/utils.h"
#include "../util/filesystem_utils.h"

class DiskArrayChar;

class DiskCharArray: public ICharArray<DiskArrayChar> {
public:
    DiskCharArray();
    DiskCharArray(string fname);
    ~DiskCharArray();

    DiskArrayChar operator[](size_t i);
    bool allocate(size_t size);
    bool resize(size_t size);
    /** Effectively, reset the object by deleting the file, setting array
     * size to 0, and creating new empty file for reading/writing. */
    void freeMemory();
    bool setChars(char const* chars, size_t N);

    bool persist(string f);
    /** Load array from file or folder, old file will be deleted. */
    bool load(string f);
    /** Write object state and (in-file) chars to the stream.
     * Object's state remains unchanged. */
    void writeToStream(ostream& stream);
    /** Read object's data from the stream.
     * Data is copied to the file currently used for storage and the old data is discarded. */
    void readFromStream(istream& stream);

    static const string PERSIST_CHARS_FNAME;

    friend class DiskArrayChar;

private:
    size_t numOfBlocks;
    FILE *file;
    char *iobuffer; // handled by openFile/closeFile methods
    string fname;
    bool fileOpened;
    int state;

    static const bool DEBUG = false;
    static const size_t BUFFER_SIZE = 1024; // if set to 0, no buffering is used
    static const string PERSIST_FIELDS_FNAME;

    // indicators of object's state
    // TODO fully implement state handling: char read/write, open, close ...
    static const int STATE_CLOSED = 0; // file closed, object should not be used anymore
    static const int STATE_OPENED = 1; // file opened for read/write
    static const int STATE_OPENED_READONLY = 2; // file opened for reading
    static const int STATE_ERROR = 3; // file I/O error or other error
    // object use is disabled, because it was persisted to the same location
    // where the file resides - therefore, the file must not be changed
    // it should be explicitly re-opened for subsequent usage
    static const int STATE_PERSISTED = 4;
    // loaded from and bound to previously persisted folder location, file must not be deleted
    static const int STATE_LOADED = 5;

    bool closeFile();
    bool openFile();
    bool flushFile();
    bool deleteFile();
    bool fileExists();
    bool createEmptyFile();
    void bindToRandomFile();
    void writeCharacter(size_t index, char ch);
    char readCharacter(size_t index);

    // for persistence, read/write object state-holding fields to/from stream
    void writeFieldsToStream(ostream& stream);
    void readFieldsFromStream(istream& stream);
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
