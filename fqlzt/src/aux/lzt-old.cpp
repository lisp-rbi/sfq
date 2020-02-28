#include "lzt-old.hpp"


extern "C"
LztOld::LztOld(){
}

LztOld::~LztOld(){
    if (trie != NULL) freeTrieMemory(trie);
}

bool LztOld::make(TSymbol* words, long length, string savePath, bool sortWords) {
    return createLzTrie(words, length, savePath, sortWords);
}

bool LztOld::read(string triePath) {
    trie = loadLzTrie(triePath);
    return trie != NULL;
}

vector<vector<TSymbol> >* LztOld::getFastqRecords(vector<TSymbol> prefix) {
    return queryLzTrie(trie, prefix);
}
