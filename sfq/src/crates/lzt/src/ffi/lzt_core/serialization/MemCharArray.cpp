#include <fstream>

#include "MemCharArray.h"

const string MemCharArray::PERSIST_FNAME = "MemCharArray.bin";

MemCharArray::MemCharArray(): numOfBlocks(0), blocks(NULL) { }

bool MemCharArray::allocate(size_t size) {
    blocks = (char *)malloc(size);    
    if (blocks != NULL) numOfBlocks = size;
    else numOfBlocks = 0;
    return blocks != NULL;
}

bool MemCharArray::resize(size_t size) {
    void *newblocks = realloc(blocks, size);
    if (newblocks != NULL) {
        blocks = (char *)newblocks;
        numOfBlocks = size;
        return true;
    }
    else return false;        
}

void MemCharArray::freeMemory() {
    if (blocks != NULL) {        
        free(blocks);
        blocks = NULL;
        numOfBlocks = 0;
    }
}

bool MemCharArray::setChars(char* chars, size_t N, bool copy) {    
    freeMemory();
    if (copy) {
        if (!allocate(N)) return false;
        for (size_t i = 0; i < N; ++i) blocks[i] = chars[i];
        return true;
    }
    else {
        numOfBlocks = N;
        blocks = chars;
    }
}

void MemCharArray::writeToStream(ostream& stream) {
    SerializationUtils::integerToStream(numOfBlocks, stream);    
    stream.write(blocks, numOfBlocks);
}

void MemCharArray::readFromStream(istream& stream) {
    numOfBlocks = SerializationUtils::integerFromStream<size_t>(stream);    
    allocate(numOfBlocks);
    stream.read(blocks, numOfBlocks);
}

bool MemCharArray::persist(string f) {
    string fname = accessible_filename(f, PERSIST_FNAME);
    if (fname == "") return false;
    ofstream output(fname.c_str());
    writeToStream(output);
    output.close();    
    return output.good();
}

bool MemCharArray::load(string f) {
    string fname = accessible_filename(f, PERSIST_FNAME);
    if (fname == "") return false;
    ifstream stream(fname.c_str());
    if (stream.good()) {  
        freeMemory();
        readFromStream(stream);
        stream.close();
        if (!stream.bad()) return true;
        else return false;
    }
    else return false;    
}

MemCharArray::~MemCharArray() { }

