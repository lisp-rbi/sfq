#include "lzt.hpp"

Lzt::Lzt(){
}

Lzt::~Lzt(){
    if (trie != NULL) freeTrieMemory(trie);
}

bool Lzt::make(TSymbol* words, long length, string savePath, bool sortWords) {
    return createLzTrie(words, length, savePath, sortWords);
}

bool Lzt::read(string triePath) {
    trie = loadLzTrie(triePath);
    return trie != NULL;
}

vector<vector<TSymbol> >* Lzt::getFastqRecords(vector<TSymbol> prefix) {
    return queryLzTrie(trie, prefix);
}