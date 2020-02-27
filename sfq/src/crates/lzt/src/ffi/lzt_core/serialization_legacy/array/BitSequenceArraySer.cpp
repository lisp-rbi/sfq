#include "BitSequenceArraySer.h"
#include "../SerializationUtils.h"
#include <cassert>

/** Serializes BitSequenceArray to output stream. */
void BitSequenceArraySerL::arrayToStream(BitSequenceArrayL const & array, ostream& stream) {
    // write integer members
    SerializationUtils::integerToStream(array.numOfBlocks, stream);
    SerializationUtils::integerToStream(array.numOfSequences, stream);
    SerializationUtils::integerToStream(array.bitsPerSequence, stream);

    // write the array
    stream.write(array.getBlocks(), array.getNumOfBlocks());
}

//TODO make the method write directly to an array
/** Deserializes BitSequenceArray from input stream. */
void BitSequenceArraySerL::arrayFromStream(BitSequenceArrayL &array, istream& stream) {
    array.freeBlocks();

    // read integer members
    array.numOfBlocks = SerializationUtils::integerFromStream<size_t>(stream);
    array.numOfSequences = SerializationUtils::integerFromStream<size_t>(stream);
    array.bitsPerSequence = SerializationUtils::integerFromStream<size_t>(stream);

    // read the array
    array.allocateBlocks();
    stream.read(array.blocks, array.numOfBlocks);
}
