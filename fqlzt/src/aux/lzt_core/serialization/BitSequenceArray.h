#ifndef BITSEQUENCEARRAY_H
#define	BITSEQUENCEARRAY_H

#include <cstddef>

#include "../serialization_legacy/BitSequence.h"
#include "../serialization_legacy/BitPointer.h"
#include "../serialization_legacy/serialization.h"

// class BitSequenceArraySerL;

/** Array for space efficient storing and fetching of large number
 * of BitSequences with predefined size, with the ability to export as chars. */
template <typename TCharArray>
class  BitSequenceArray {
public:

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
    char const * getBlocks() const;
    size_t getNumOfBlocks() const;    

    size_t getNumOfSequences() const;
    size_t getSequenceSize() const;
    TCharArray* exportCharArray();

    // Friend so that it can serialzie/deserialze the array
    // friend class BitSequenceArraySerL;

private:

    void allocateBlocks();
    void freeBlocks();

    static int const BLOCK_SIZE = sizeof(char);
    static char const ONE = 1;

    size_t numOfBlocks = 0;
    size_t numOfSequences = 0;
    size_t bitsPerSequence = 0;

    char *blocks = NULL;
    TCharArray* charArray;
    bool carrayExported = false;

    void nullArray();

};

/** Get index-th bit sequence, index is zero-based.
 * ONLY THE FIRST sequenceSize BITS WILL BE SET set,
 * other bits of the returned BitSequence are left uninitialzed. */
template <typename TCharArray>
inline BitSequence BitSequenceArray<TCharArray>::operator[](size_t index) const {
    BitSequence seq;
    // pointer to the first bit of the i-th stored sequence
    BitPointer bp(1, index * bitsPerSequence);
    char block = blocks[bp.blockIndex];

    for (int i = 0; i < bitsPerSequence; ++i) {
        // if increment advanced to the next block, get it
        if (bp.bitIndex == 0)
            block = blocks[bp.blockIndex];

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
BitSequenceArray<TCharArray>::BitSequenceArray(size_t size, int bitsPerSeq) {
    changeFormat(size, bitsPerSeq);
}

template <typename TCharArray>
BitSequenceArray<TCharArray>::BitSequenceArray(const BitSequenceArray& src)
: numOfBlocks(src.numOfBlocks), numOfSequences(src.numOfSequences),
  bitsPerSequence(src.bitsPerSequence) {
    freeBlocks();
    allocateBlocks();
    for (size_t i = 0; i < numOfBlocks; ++i) blocks[i] = src.blocks[i];
}

template <typename TCharArray>
BitSequenceArray<TCharArray>::~BitSequenceArray() {
    // ! carrayExported
    freeBlocks();
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

/** Resize the array so that it can store newSize sequences, keeping the
 * values of the sequences up to the lesser of two sizes. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::resize(size_t newSize) {
    numOfSequences = newSize;
    size_t numOfBits = numOfSequences * bitsPerSequence;
    numOfBlocks = numberOfBlocks(numOfBits, BLOCK_SIZE);
    blocks = (char *)realloc(blocks, numOfBlocks);
}

/** Change size and/or bitsPerSequence to new values, reallocating if
 * necessary, if the bitsPerSeq changed, values in the new array are undefinded,
 * else the array is just resized. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::changeFormat(size_t size, int bitsPerSeq) {
    freeBlocks();
    bitsPerSequence = bitsPerSeq; numOfSequences = size;
    size_t numOfBits = numOfSequences * bitsPerSequence;
    numOfBlocks = numberOfBlocks(numOfBits, BLOCK_SIZE);
    allocateBlocks();
}

template <typename TCharArray>
BitSequenceArray<TCharArray>& BitSequenceArray<TCharArray>::operator=(const BitSequenceArray<TCharArray>& rhs) {
    if (this == &rhs) return *this;

    numOfBlocks = rhs.numOfBlocks;
    numOfSequences = rhs.numOfSequences;
    bitsPerSequence = rhs.bitsPerSequence;

    freeBlocks();
    allocateBlocks();
    for (size_t i = 0; i < numOfBlocks; ++i) blocks[i] = rhs.blocks[i];

    return *this;
}

/** Reallocate memory for numOfBlocks blocks. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::allocateBlocks() {
    blocks = (char *)malloc(numOfBlocks);
}

/** Free blocks memory. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::freeBlocks() {
 if (blocks != NULL) {
     free(blocks);
     blocks = NULL;
 }
}

/** Set all bits in the array to 0. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::nullArray() {
    for (size_t i = 0; i < numOfBlocks; ++i)
        blocks[i] = zeroBits<char>();    
}

/** Set i-th bit sequence to seq, i is zero-based. */
template <typename TCharArray>
void BitSequenceArray<TCharArray>::setSequence(size_t index, BitSequence seq) {
    // pointer to the bit in the array where first bit of the seq will be stored
    BitPointer bp(1, index * bitsPerSequence);

    for (int i = 0; i < bitsPerSequence; ++i) {
        bool bit = seq[i];
        
        if (bit)
            blocks[bp.blockIndex] |= (ONE << bp.bitIndex);
        else
            blocks[bp.blockIndex] &= ~(ONE << bp.bitIndex);

        bp.increment();
    }
}

template <typename TCharArray>
char const * BitSequenceArray<TCharArray>::getBlocks() const {
    return blocks;
}

template <typename TCharArray>
size_t BitSequenceArray<TCharArray>::getNumOfBlocks() const {
    return numOfBlocks;
}

#endif	/* BITSEQUENCEARRAY_H */

