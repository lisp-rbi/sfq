#ifndef BITSEQUENCEARRAYLEGACY_H
#define	BITSEQUENCEARRAYLEGACY_H

#include <cstddef>

#include "BitSequence.h"
#include "BitPointer.h"
#include "serialization.h"

class BitSequenceArraySerL;

/** Array for space efficient storing and fetching of large number
 * of BitSequences with predefined size, with the ability to export as chars. */
class BitSequenceArrayL {
public:

    BitSequenceArrayL();
    BitSequenceArrayL(size_t size, int bitsPerSequence);
    BitSequenceArrayL(const BitSequenceArrayL& orig);
    BitSequenceArrayL& operator=(const BitSequenceArrayL& rhs);
    virtual ~BitSequenceArrayL();   

    BitSequence operator[](size_t i) const;
    void changeFormat(size_t size, int bitsPerSequence);
    void resize(size_t newSize);
    void setSequence(size_t i, BitSequence seq);
    char const * getBlocks() const;
    size_t getNumOfBlocks() const;

    size_t getNumOfSequences() const;
    size_t getSequenceSize() const;

    // Friend so that it can serialzie/deserialze the array
    friend class BitSequenceArraySerL;
    template <typename TS, typename TI, typename TBitSequenceArray> friend class CompactArrayBuilder;

private:

    void allocateBlocks();
    void freeBlocks();

    static int const BLOCK_SIZE = sizeof(char);
    static char const ONE = 1;

    size_t numOfBlocks;
    size_t numOfSequences;
    size_t bitsPerSequence;

    char *blocks;

    void nullArray();

};

/** Get index-th bit sequence, index is zero-based.
 * ONLY THE FIRST sequenceSize BITS WILL BE SET set,
 * other bits of the returned BitSequence are left uninitialzed. */
inline BitSequence BitSequenceArrayL::operator[](size_t index) const {
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

inline size_t BitSequenceArrayL::getNumOfSequences() const {
    return numOfSequences;
}

inline size_t BitSequenceArrayL::getSequenceSize() const {
    return bitsPerSequence;
}

/** Hash map-set equality function, two indexes are equal iff
 * they point to equal sequences in a BitSequenceArray. */
class BSAEquals {
public:
    BSAEquals(const BitSequenceArrayL& a);
    inline bool operator()(size_t i1, size_t i2) const;
private:
    const BitSequenceArrayL& array;
    int bits;
};

#endif	/* BITSEQUENCEARRAYLEGACY_H */

