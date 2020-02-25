#ifndef COMPACTARRAY_H
#define	COMPACTARRAY_H

#include <cstddef>
#include <cassert>
#include <iostream>

#include "CompactSymbolArray.h"
#include "serialization/ISerializable.h"
#include "node_array/compact_array_legacy/utils.h"
#include "node_array/compact_array_legacy/CompactArrayNode.h"
#include "serialization_legacy/BitSequenceArray.h"
#include "serialization_legacy/serialization.h"

/** Implementation of const NodeArray concept.
 * It's a node array in wich only distinct nodes are stored and the array is a
 * sequence of indexes pointing to these nodes. Eow and Cow flags are not
 * stored, they are calculated from index position. */
template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
class CompactArray : ISerializable {

public:
       
    CompactArray(bool enumerated = false);
    virtual ~CompactArray();

    typedef TSymbol Symbol;
    typedef TIndex Index;
    typedef CompactArrayNode<TSymbol, TIndex> NodeConst;

    //TODO promjeniti tip parametra u size_t
    NodeConst operator[](TIndex i) const;
    TIndex getSize() const;
    bool isEnumerated() const { return enumerated; }
    // serialization
    bool persist(string f);
    bool load(string f);
    void writeToStream(ostream& stream);
    void readFromStream(istream& stream);
    // file and folder locations for serialized substructures
    static const string ARRAY_FOLDER;
    static const string SIBLINGS_FOLDER;
    static const string NUMOFWORDS_FOLDER;        
    static const string SYMBOLS_FOLDER;

    template <typename TS, typename TI, typename TBSA> friend class CompactArrayBuilder;

private:
    
    CompactArray(size_t numOfDistinct, size_t numOfNodes, bool enumerated = false);

    static const int NUM_OFFSETS = 4;

    void reserveNodeSpace(size_t size);
    void setFlagOffsets(size_t flagOffsets[NUM_OFFSETS]);
    void setNodeIndex(size_t position, size_t nodeIndex);

    int flagsFromPosition(size_t p) const;

    void printIndexes() const;
    
    // serialization
    void writeFieldsToStream(ostream& stream);
    void readFieldsFromStream(istream& stream);
    void writeSubstructsToStream(ostream& stream);
    void readSubstructsFromStream(istream& stream);   
    bool persistSubstructures(string directory);    
    bool loadSubstructures(string directory);    
    static const string PERSIST_FIELDS_FNAME;

    size_t numOfDistinct;
    size_t numOfNodes;
    size_t bitsPerIndex;

    // true if the numOfWords is stored for each node
    bool enumerated;

    /* Cow and eow flags are deduced from index, these are the borders of
     * blocks of nodes with same flag combinations. For each combination (coded
     * as number in [0,3]), nodes with that flag values lie in the interval
     * [ flagOffsets[flags], flagOffsets[flags+1] - 1 ]. */
    size_t flagOffsets[NUM_OFFSETS];

    TBitSequenceArray array;    
    TBitSequenceArray siblings;
    // number of words a node contains
    TBitSequenceArray numOfWords;
    CompactSymbolArray<TSymbol, TBitSequenceArray> symbols;
    
};

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
const string CompactArray<TSymbol, TIndex, TBitSequenceArray>::PERSIST_FIELDS_FNAME = "CompactArrayFields.bin";

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
const string CompactArray<TSymbol, TIndex, TBitSequenceArray>::ARRAY_FOLDER = "array";

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
const string CompactArray<TSymbol, TIndex, TBitSequenceArray>::SIBLINGS_FOLDER = "siblings";

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
const string CompactArray<TSymbol, TIndex, TBitSequenceArray>::NUMOFWORDS_FOLDER = "numofwords";

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
const string CompactArray<TSymbol, TIndex, TBitSequenceArray>::SYMBOLS_FOLDER = "symbols";

int numberOfBits(size_t numberOfValues);

/** Default constructor, safe to use only for deserialization. */
template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
CompactArray<TSymbol, TIndex, TBitSequenceArray>::CompactArray(bool e)
: numOfDistinct(0), numOfNodes(0), bitsPerIndex(0), enumerated(e) { }

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
CompactArray<TSymbol, TIndex, TBitSequenceArray>::CompactArray(size_t distinct, size_t nodes, bool e)
: numOfDistinct(distinct), numOfNodes(nodes), enumerated(e),
   bitsPerIndex(numberOfBits(distinct)), array(nodes, bitsPerIndex)
{ }

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
CompactArray<TSymbol, TIndex, TBitSequenceArray>::~CompactArray() { }

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
TIndex CompactArray<TSymbol, TIndex, TBitSequenceArray>::getSize() const {
    return numOfNodes;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::printIndexes() const {
    for (size_t i = 0; i < numOfNodes; ++i) {
        size_t index = fromBitSequence<size_t>(array[i]);
        cout<<index<<endl;
    }
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
inline CompactArrayNode<TSymbol, TIndex>
CompactArray<TSymbol, TIndex, TBitSequenceArray>::operator[](TIndex i) const {
    CompactArrayNode<TSymbol, TIndex> node;
    // decode node-table index of a node
    BitSequence indexBits = array[i];
    size_t index = fromBitSequence<size_t>(indexBits, bitsPerIndex);

    // get silbing and symbol data from the table
    node.sibling = numberFromBits<TIndex>(siblings[index], siblings.getSequenceSize());
    // TODO TIndex type operation (conversion to size_t)
    node.symbol = symbols[(size_t)index];

    // calculate flag values
    int flags = flagsFromPosition(index);
    node.eow = intFlagsEow(flags);
    node.cow = intFlagsCow(flags);

    // get numberOfWords if array is enumerated
    if (enumerated) {
        node.numOfWords = numberFromBits<TIndex>(numOfWords[index], numOfWords.getSequenceSize());
        node.enumerated = true;
    }
    else node.enumerated = false;

    return node;
}

/** Set nodeIndex on position i in the BitSequenceArray, wich means
 * that i-th node in the array will be the distinct node at position nodeIndex */
template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::setNodeIndex(size_t i, size_t nodeIndex) {    
    array.setSequence(i, toBitSequence(nodeIndex, bitsPerIndex));
}

/** Calculate int representation of eow-cow, from position
 * in distinct nodes array.  */
template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
int CompactArray<TSymbol, TIndex, TBitSequenceArray>::flagsFromPosition(size_t p) const {
    for (int i = 0; i < NUM_OFFSETS; ++i) {
        size_t start = flagOffsets[i], end;
        // calculate end of range for current flags
        if (i < NUM_OFFSETS - 1) end = flagOffsets[i+1];
        else end = numOfDistinct;

        if (start <= p && p < end) return i;
    }
    // flags must be calculated
    assert(false);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::setFlagOffsets(size_t offsets[NUM_OFFSETS]) {
    for (int i = 0; i < NUM_OFFSETS; ++i) flagOffsets[i] = offsets[i];
}


template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
bool CompactArray<TSymbol, TIndex, TBitSequenceArray>::persist(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {
            ofstream output(f.c_str());
            writeToStream(output);
            output.close();    
            return output.good();
        }
        else if (file_is_directory(f)) {                
            // persist fields
            string fname = accessible_filename(f, PERSIST_FIELDS_FNAME);
            if (fname == "") return false;
            ofstream fieldsstr(fname.c_str());
            writeFieldsToStream(fieldsstr); fieldsstr.close();            
            if (!fieldsstr.good()) return false;            
            return persistSubstructures(f);
        }
        else return false;
    }
    else return false;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
bool CompactArray<TSymbol, TIndex, TBitSequenceArray>::load(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {            
            ifstream stream(f.c_str());
            if (stream.good()) {  
                readFromStream(stream);
                stream.close();
                return stream.good();
            }
            else return false;    
        }
        else if (file_is_directory(f)) {          
            // load fields            
            string fname = accessible_filename(f, PERSIST_FIELDS_FNAME);
            if (fname == "") return false;
            ifstream fieldsstr(fname.c_str());
            readFieldsFromStream(fieldsstr); fieldsstr.close();
            if (!fieldsstr.good()) return false;
            return loadSubstructures(f);
        }
        else return false;
    }
    else return false;    
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::writeToStream(ostream& stream) {
    writeFieldsToStream(stream);
    writeSubstructsToStream(stream);    
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::readFromStream(istream& stream) {
    readFieldsFromStream(stream);
    readSubstructsFromStream(stream);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::writeFieldsToStream(ostream& stream) {    
    SerializationUtils::integerToStream(numOfDistinct, stream);    
    SerializationUtils::integerToStream(numOfNodes, stream);    
    SerializationUtils::integerToStream(bitsPerIndex, stream);        
    SerializationUtils::integerToStream(enumerated, stream);        
    SerializationUtils::integerToStream(NUM_OFFSETS, stream);   
    for (int i = 0; i < NUM_OFFSETS; ++i)
        SerializationUtils::integerToStream(flagOffsets[i], stream);    
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::readFieldsFromStream(istream& stream) {    
    // numOfDistinct = SerializationUtils::integerFromStream<size_t>(stream); 
    numOfDistinct = SerializationUtils::integerFromStream<size_t>(stream);
    numOfNodes = SerializationUtils::integerFromStream<size_t>(stream);
    bitsPerIndex = SerializationUtils::integerFromStream<size_t>(stream);
    enumerated = SerializationUtils::integerFromStream<bool>(stream);
    int numOff = SerializationUtils::integerFromStream<int>(stream);
    assert(numOff == NUM_OFFSETS);
    for (int i = 0; i < NUM_OFFSETS; ++i)
        flagOffsets[i] = SerializationUtils::integerFromStream<size_t>(stream);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::writeSubstructsToStream(ostream& stream) {
    array.writeToStream(stream);
    siblings.writeToStream(stream);
    numOfWords.writeToStream(stream);
    symbols.writeToStream(stream);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArray<TSymbol, TIndex, TBitSequenceArray>::readSubstructsFromStream(istream& stream) {
    array.readFromStream(stream);
    siblings.readFromStream(stream);
    numOfWords.readFromStream(stream);
    symbols.readFromStream(stream);
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
bool CompactArray<TSymbol, TIndex, TBitSequenceArray>::persistSubstructures(string directory) {
    string sep = "/"; bool res;
    string arrayFolder = directory+sep+ARRAY_FOLDER;
    res = create_directory(arrayFolder) and array.persist(arrayFolder);
    if (!res) return false;
    string siblFolder = directory+sep+SIBLINGS_FOLDER;
    res = create_directory(siblFolder) and siblings.persist(siblFolder); 
    if (!res) return false;
    string numofwFolder = directory+sep+NUMOFWORDS_FOLDER;
    res = create_directory(numofwFolder) and numOfWords.persist(numofwFolder);   
    if (!res) return false;
    string symbFolder = directory+sep+SYMBOLS_FOLDER;
    res = create_directory(symbFolder) and symbols.persist(symbFolder);               
    if (!res) return false;
    return true;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
bool CompactArray<TSymbol, TIndex, TBitSequenceArray>::loadSubstructures(string directory) {
    string sep = "/"; bool res;
    string arrayFolder = directory+sep+ARRAY_FOLDER;
    res = array.load(arrayFolder); if (!res) return false;
    string siblFolder = directory+sep+SIBLINGS_FOLDER;
    res = siblings.load(siblFolder); if (!res) return false;
    string numofwFolder = directory+sep+NUMOFWORDS_FOLDER;
    res = numOfWords.load(numofwFolder);  if (!res) return false;
    string symbFolder = directory+sep+SYMBOLS_FOLDER;
    res = symbols.load(symbFolder); if (!res) return false;
    return true;
}


#endif	/* COMPACTARRAY_H */

