#include "lzt.hpp"


extern "C"
Lzt::Lzt(){
}

Lzt::~Lzt(){
    if (trie != NULL) freeTrieMem(trie);
}

bool Lzt::make(TSymbol* words, long length, string savePath, bool sortWords) {
    return createTrie(words, length, savePath, sortWords);
}

bool Lzt::read(string triePath, bool inMem) {
    trie = loadTrie(triePath, inMem);
    return trie != NULL;
}

vector<vector<TSymbol> >* Lzt::getFastqRecords(vector<TSymbol> prefix) {
    return queryTrie(trie, prefix);
}

void Lzt::setCacheSize(size_t size) {
    trie->nodes.setCache(size);
}
