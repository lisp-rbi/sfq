
#ifndef LZT_INTERFACE_OLD_H
#define LZT_INTERFACE_OLD_H

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
#include "lzt_core/node_array/compact_array_legacy/CompactArray.h"
#include "lzt_core/node_array/compact_array_legacy/CompactArrayCreator.h"
#include "lzt_core/serialization_legacy/array/CompactArraySerializer.h"

/**************** DEFINITION OF TEMPLATED AND UTIL TYPES ****************/
typedef long TIndex;
typedef unsigned char TSymbol;
//typedef short int TSymbol;
//typedef int TSymbol;

typedef VectorArray<TSymbol, TIndex> TNodeArray; // used in compression
typedef CompactArrayL<TSymbol, TIndex> TCompactArrayL; // final smaller representation
typedef LzTrie<CompactArrayL<TSymbol, TIndex> > TLzTrieL; // final compressed trie
/**************************************************************/

/**************** INTERFACE FUNCTIONS ****************/
bool createLzTrie(TSymbol* words, long length, string fname, bool sortWords = false);
TLzTrieL* loadLzTrie(string trieFile);
vector<vector<TSymbol> >* queryLzTrie(TLzTrieL* trie, vector<TSymbol> query);
void freeTrieMemory(TLzTrieL* trie);
/**************************************************************/

#endif /* LZT_INTERFACE_OLD_H */

