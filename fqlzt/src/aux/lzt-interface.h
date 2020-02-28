
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
#include "lzt_core/node_array/compact_array/CompactArray.h"
#include "lzt_core/node_array/compact_array/CompactArrayBuilder.h"
#include "lzt_core/serialization/BitSequenceArray.h"
#include "lzt_core/serialization/DiskCharArray.h"

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
typedef CompactArray<TSymbol, TIndex, TBitSeqArrayDisk> TCompactArrayDisk; 
typedef LzTrie<TCompactArrayDisk> TLzTrieDisk; // final compressed trie
/**************************************************************/

/**************** INTERFACE FUNCTIONS ****************/
bool createTrie(TSymbol* words, long length, string fname, bool sortWords = false);
TLzTrieDisk* loadTrie(string trieFolder);
vector<vector<TSymbol> >* queryTrie(TLzTrieDisk* trie, vector<TSymbol> query);
void freeTrieMem(TLzTrieDisk* trie);
/**************************************************************/

#endif /* LZT_INTERFACE_H */

