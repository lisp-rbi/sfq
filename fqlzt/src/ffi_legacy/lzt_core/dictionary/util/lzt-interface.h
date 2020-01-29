
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
#include "lzt_core/node_array/compact_array/CompactArrayCreator.h"
#include "lzt_core/serialization/array/CompactArraySerializer.h"

/**************** DEFINITION OF IMPORTANT TEMPLATED TYPES ****************/
typedef long TIndex;
typedef unsigned char TSymbol;
//typedef short int TSymbol;
//typedef int TSymbol;

typedef VectorArray<TSymbol, TIndex> TNodeArray; // used in compression
typedef CompactArray<TSymbol, TIndex> TCompactArray; // final smaller representation
typedef LzTrie<CompactArray<TSymbol, TIndex> > TLzTrie; // final compressed trie
/**************************************************************/

/**************** INTERFACE FUNCTIONS ****************/
bool createLzTrie(vector<vector<TSymbol> >* words, string fname, bool sortWords = false);
TLzTrie* loadLzTrie(string trieFile);

vector<vector<TSymbol> >* queryLzTrie(TLzTrie* trie, vector<TSymbol> query);
void freeTrieMemory(TLzTrie* trie);

/**************************************************************/

/**************** HELPER FUNCTIONS ****************/
WordList<TSymbol>* vecOfVec2WordList(vector<vector<TSymbol> >* words);
vector<vector<TSymbol> >* wordList2VecOfVec(WordList<TSymbol>* words);
vector<vector<TSymbol> >* readWordsFromFile(string file);
TSymbol* symbolVec2array(vector<TSymbol> w);
string symbolVec2string(vector<TSymbol> w);
/**************************************************************/

#endif /* LZT_INTERFACE_H */
