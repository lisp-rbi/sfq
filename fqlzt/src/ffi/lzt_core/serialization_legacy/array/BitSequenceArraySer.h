#ifndef BITSEQUENCEARRAYSER_H
#define	BITSEQUENCEARRAYSER_H

#include <iostream>

#include "../BitSequenceArray.h"
#include "../BitSequence.h"

/** Serializes BitSequenceArray to and deserializes from stream objects. */
class BitSequenceArraySerL {
public:
    virtual ~BitSequenceArraySerL() {};
    //TODO add serialization test
    static void arrayToStream(BitSequenceArrayL const & array, ostream & stream);
    static void arrayFromStream(BitSequenceArrayL &array, istream& stream);

private:

};

#endif	/* BITSEQUENCEARRAYSER_H */
