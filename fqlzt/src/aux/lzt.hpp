#ifndef LZT_HPP
#define LZT_HPP

#include "lzt-interface.h"

#include <iostream>
#include <cstddef>
extern "C"
using namespace std;

class Lzt {

    public:
	Lzt();
	~Lzt();

        /**
         * Creates compressed trie from a sorted list of words
         * assumes dict does not exist and overwrites it
         */
        bool make(TSymbol* words, long length, string savePath, bool sortWords=false);

        /**
         *  If trie exists, load the data structure into self
         *  Return true if loading is successful, else false.
         */
        bool read(string triePath, bool inMem=false);

        /**
         * Query (loaded) trie by prefix, return a list of all words sharing the prefix.
         * Empty prefix lists all words.
         */
        vector<vector<TSymbol> > * getFastqRecords(vector<TSymbol> prefix);

    private:
        TLzTrie *trie = NULL;
};


#endif
