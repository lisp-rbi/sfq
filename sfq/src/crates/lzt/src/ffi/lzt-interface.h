
#ifndef LZT_INTERFACE_H
#define LZT_INTERFACE_H

#include <cstdlib>
#include <cstddef>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

#include "lzt_core/util/WordFileReader.h"
#include "lzt_core/util/factory.h"
#include "lzt_core/dictionary/util/WordList.h"
#include "lzt_core/node_array/vector_array/VectorArray.h"
#include "lzt_core/node_array/compact_array/ICompactArray.h"
#include "lzt_core/node_array/compact_array_legacy/CompactArrayNode.h"
#include "lzt_core/node_array/compact_array/CompactArray.h"
#include "lzt_core/node_array/compact_array/CompactArrayBuilder.h"
#include "lzt_core/serialization/BitSequenceArray.h"
#include "lzt_core/serialization/DiskCharArray.h"
#include "lzt_core/serialization/MemCharArray.h"

#include "lzt_core/node_array/compact_array_legacy/CompactArray.h"
#include "lzt_core/node_array/compact_array_legacy/CompactArrayCreator.h"
#include "lzt_core/serialization_legacy/array/CompactArraySerializer.h"

/**************** DEFINITION OF TEMPLATED AND UTIL TYPES ****************/
typedef long TIndex;
typedef unsigned char TSymbol;
//typedef short int TSymbol;
//typedef int TSymbol;

// array used in compression
typedef VectorArray<TSymbol, TIndex> TNodeArray;
// final smaller representation
typedef BitSequenceArray<DiskCharArray> TBitSeqArrayDisk;
typedef BitSequenceArray<MemCharArray> TBitSeqArrayMem;
typedef CompactArray<TSymbol, TIndex, TBitSeqArrayDisk> TCompactArrayDisk;
typedef CompactArray<TSymbol, TIndex, TBitSeqArrayMem> TCompactArrayMem;
typedef ICompactArray<TSymbol, TIndex, CompactArrayNode<TSymbol, TIndex> > TCompactArray;
typedef LzTrie<TCompactArrayDisk> TLzTrieDisk; // final compressed trie
typedef LzTrie<TCompactArray> TLzTrie; // final compressed trie
/**************************************************************/

/**************** INTERFACE FUNCTIONS ****************/
bool createTrie(TSymbol* words, long length, string fname, bool sortWords);
TLzTrie* loadTrie(string trieFolder, bool mem);
vector<TSymbol > queryTrie(TLzTrie* trie, vector<TSymbol> query);
void freeTrieMem(TLzTrie* trie);
/**************************************************************/

#endif /* LZT_INTERFACE_H */
