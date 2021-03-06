#ifndef BITSEQUENCEARRAY_H
#define	BITSEQUENCEARRAY_H

#include <cstddef>
#include <iostream>
#include <fstream>

#include "ISerializable.h"
#include "serialization_legacy/BitSequence.h"
#include "serialization_legacy/BitPointer.h"
#include "serialization_legacy/serialization.h"
#include "serialization_legacy/SerializationUtils.h"
#include "util/filesystem_utils.h"

using namespace std;

/** Array for space efficient storing and fetching of large number
 * of BitSequences with predefined size, with the ability to export as chars. */
template <typename TCharArray>
class  BitSequenceArray : public ISerializable {
public:

    typedef TCharArray ArrayType;
    
    BitSequenceArray();
    BitSequenceArray(TCharArray* charArray);
    BitSequenceArray(size_t size, int bitsPerSequence);
    BitSequenceArray(const BitSequenceArray& orig);
    BitSequenceArray& operator=(const BitSequenceArray& rhs);
    virtual ~BitSequenceArray();   

    BitSequence operator[](size_t i) const;
    void changeFormat(size_t size, int bitsPerSequence);
    void resize(size_t newSize);
    void setSequence(size_t i, BitSequence seq);    

    size_t getNumOfSequences() const;
    size_t getSequenceSize() const;
    TCharArray* exportCharArray();
    void setCharArray(TCharArray* charArray);
    // bool setCharArrayChars(char const* chars, size_t N, bool copy=true);

    bool persist(string f);
    bool load(string f);
    void writeToStream(ostream& stream);
    void readFromStream(istream& stream);
    
    template <typename TS, typename TI, typename TBitSequenceArray> friend class CompactArrayBuilder;

private:

    void writeFieldsToStream(ostream& stream);
    void readFieldsFromStream(istream& stream);
    
    static int const BLOCK_SIZE = sizeof(char);
    static char const ONE = 1;
    static const string PERSIST_FNAME;

    size_t numOfBlocks = 0;
    size_t numOfSequences = 0;
    size_t bitsPerSequence = 0;

    // char *blocks = NULL;
    TCharArray* charArray;
    bool carrayExported = false;

};

template <typename TCharArray>
const string BitSequenceArray<TCharArray>::PERSIST_FNAME = "BitSequenceArray.bin";

/** Get index-th bit sequence, index is zero-based.
 * ONLY THE FIRST sequenceSize BITS WILL BE SET set,
 * other bits of the returned BitSequence are left uninitialzed. */
template <typename TCharArray>
inline BitSequence BitSequenceArray<TCharArray>::operator[](size_t index) const {
    BitSequence seq;
    // pointer to the first bit of the i-th stored sequence
    BitPointer bp(1, index * bitsPerSequence);
    char block = (*charArray)[bp.blockIndex]; // blocks[bp.blockIndex];

    for (int i = 0; i < bitsPerSequence; ++i) {
        // if increment advanced to the next block, get it
        if (bp.bitIndex == 0)
            block =  (*charArray)[bp.blockIndex]; // blocks[bp.blockIndex];

        bool bit = (bool)(block & (ONE << bp.bitIndex));
        seq.setBit(i, bit);

        bp.increment();
    }

    return seq;
}

/** Create empty array. */
template <typename TCharArray>
BitSequenceArray<TCharArray>::BitSequenceArray() { 
    charArray = new TCharArray();     
}

template <typename TCharArray>
BitSequenceArray<TCharArray>::BitSequenceArray(TCharArray* carray): charArray(carray), carrayExported(true) {}

template <typename TCharArray>
BitSequenceArray<TCharArray>::BitSequenceArray(size_t size, int bitsPerSeq): BitSequenceArray() {
    changeFormat(size, bitsPerSeq);
}

template <typename TCharArray>
BitSequenceArray<TCharArray>::BitSequenceArray(const BitSequenceArray& src)
: numOfBlocks(src.numOfBlocks), numOfSequences(src.numOfSequences),
  bitsPerSequence(src.bitsPerSequence) {
    charArray->freeMemory(); // freeBlocks();
    charArray->allocate(numOfBlocks); // allocateBlocks();
    for (size_t i = 0; i < numOfBlocks; ++i) 
        (*charArray)[i] = (*src.charArray)[i]; // blocks[i] = src.blocks[i];
}

template <typename TCharArray>
BitSequenceArray<TCharArray>::~BitSequenceArray() {    
    // freeBlocks();
    if (carrayExported == false) charArray->freeMemory();
    delete charArray;
}

template <typename TCharArray>
inline size_t BitSequenceArray<TCharArray>::getNumOfSequences() const {
    return numOfSequences;
}

template <typename TCharArray>
inline size_t BitSequenceArray<TCharArray>::getSequenceSize() const {
    return bitsPerSequence;
}

template <typename TCharArray>
TCharArray* BitSequenceArray<TCharArray>::exportCharArray() {
    carrayExported = true;
    return charArray;
}

template <typename TCharArray>
void BitSequenceArray<TCharArray>::setCharArray(TCharArray* carray) {
    if (charArray != NULL) delete charArray;
    charArray = carray;
    carrayExported = true;
}

//template <typename TCharArray>
//bool BitSequenceArray<TCharArray>::setCharArrayChars(char const* chars, size_t N, bool copy) {
//    return charArray->setChars(chars, N, copy);
//}

/** Resize the array so that it can store newSize sequences, keeping the
 * values of the sequences up to the lesser of two sizes. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::resize(size_t newSize) {
    numOfSequences = newSize;
    size_t numOfBits = numOfSequences * bitsPerSequence;
    numOfBlocks = numberOfBlocks(numOfBits, BLOCK_SIZE);
    charArray->resize(numOfBlocks);
    // blocks = (char *)realloc(blocks, numOfBlocks);
}

/** Change size and/or bitsPerSequence to new values, reallocating if
 * necessary, if the bitsPerSeq changed, values in the new array are undefinded,
 * else the array is just resized. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::changeFormat(size_t size, int bitsPerSeq) {
    charArray->freeMemory(); // freeBlocks();
    bitsPerSequence = bitsPerSeq; numOfSequences = size;
    size_t numOfBits = numOfSequences * bitsPerSequence;
    numOfBlocks = numberOfBlocks(numOfBits, BLOCK_SIZE);
    charArray->allocate(numOfBlocks); // allocateBlocks();
}

template <typename TCharArray>
BitSequenceArray<TCharArray>& BitSequenceArray<TCharArray>::operator=(const BitSequenceArray<TCharArray>& rhs) {
    if (this == &rhs) return *this;

    numOfBlocks = rhs.numOfBlocks;
    numOfSequences = rhs.numOfSequences;
    bitsPerSequence = rhs.bitsPerSequence;

    charArray->freeMemory(); // freeBlocks();
    charArray->allocate(numOfBlocks); // allocateBlocks();
    // TODO this does not work, BitSequences are not copied by reference
    for (size_t i = 0; i < numOfBlocks; ++i) 
        (*charArray)[i] = (*rhs.charArray)[i]; // blocks[i] = rhs.blocks[i];

    return *this;
}

/** Set i-th bit sequence to seq, i is zero-based. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::setSequence(size_t index, BitSequence seq) {
    // pointer to the bit in the array where first bit of the seq will be stored
    BitPointer bp(1, index * bitsPerSequence);

    for (int i = 0; i < bitsPerSequence; ++i) {
        bool bit = seq[i];
        
        if (bit)
            (*charArray)[bp.blockIndex] |= (ONE << bp.bitIndex);
            // blocks[bp.blockIndex] |= (ONE << bp.bitIndex);
        else
            (*charArray)[bp.blockIndex] &= ~(ONE << bp.bitIndex);
            // blocks[bp.blockIndex] &= ~(ONE << bp.bitIndex);

        bp.increment();
    }
}

template <typename TCharArray>
void BitSequenceArray<TCharArray>::writeToStream(ostream& stream) {
    writeFieldsToStream(stream);
    charArray->writeToStream(stream);
}

template <typename TCharArray>
void BitSequenceArray<TCharArray>::writeFieldsToStream(ostream& stream) {
    SerializationUtils::integerToStream(numOfBlocks, stream);
    SerializationUtils::integerToStream(numOfSequences, stream);
    SerializationUtils::integerToStream(bitsPerSequence, stream);
}

template <typename TCharArray>
void BitSequenceArray<TCharArray>::readFromStream(istream& stream) {
    readFieldsFromStream(stream);
    charArray->readFromStream(stream);
}

template <typename TCharArray>
void BitSequenceArray<TCharArray>::readFieldsFromStream(istream& stream) {
    numOfBlocks = SerializationUtils::integerFromStream<size_t>(stream);
    numOfSequences = SerializationUtils::integerFromStream<size_t>(stream);
    bitsPerSequence = SerializationUtils::integerFromStream<size_t>(stream);    
}

template <typename TCharArray>
bool BitSequenceArray<TCharArray>::persist(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {
            ofstream output(f.c_str());
            writeToStream(output);            
            output.close();    
            return output.good();
        }
        else if (file_is_directory(f)) {                
            string fname = accessible_filename(f, PERSIST_FNAME);
            if (fname == "") return false;
            ofstream output(fname.c_str());
            writeFieldsToStream(output);
            output.close();
            bool arrayWrite = charArray->persist(f);
            return output.good() and arrayWrite;
        }
        else return false;
    }
    else return false;
}

template <typename TCharArray>
bool BitSequenceArray<TCharArray>::load(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {            
            ifstream stream(f.c_str());
            if (stream.good()) {  
                readFromStream(stream);
                stream.close();
                if (!stream.bad()) return true;
                else return false;
            }
            else return false;    
        }
        else if (file_is_directory(f)) {                
            string fname = accessible_filename(f, PERSIST_FNAME);
            if (fname == "") return false;
            ifstream stream(fname.c_str());
            readFieldsFromStream(stream);
            stream.close();
            bool arrayRead = charArray->load(f);
            return stream.good() and arrayRead;
        }
        else return false;
    }
    else return false;        
}


#endif	/* BITSEQUENCEARRAY_H */

