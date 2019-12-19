#include "lzt.hpp"

Lzt::Lzt(){
}

Lzt::~Lzt(){
    if (trie != NULL) freeTrieMemory(trie);
}

bool Lzt::make(vector<vector<TSymbol> >* words, string savePath) {
    return createLzTrie(words, savePath);
}

bool Lzt::read(string triePath) {
    trie = loadLzTrie(triePath);
    return trie != NULL;
}

vector<vector<TSymbol> >* Lzt::getFastqRecords(vector<TSymbol> prefix) {
    return queryLzTrie(trie, prefix);
}