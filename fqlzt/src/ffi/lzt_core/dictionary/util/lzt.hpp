#ifndef LZT_HPP
#define LZT_HPP

#include "lzt-interface.h"

#include <iostream>
#include <cstddef>

using namespace std;

extern "C" {

class Lzt {

    public:
	Lzt();
	~Lzt();

        /**
         * Creates compressed trie from a sorted list of words
         * assumes dict does not exist and overwrites it
         */
          bool make(uchar* words, long len, char* path);

        /**
         *  If trie exists, load the data structure into self
         *  Return true if loading is successful, else false.
         */
        bool read(char* path);

        /**
         * Query (loaded) trie by prefix, return a list of all words sharing the prefix.
         * Empty prefix lists all words.
         */
        long* getFastqRecords(uchar* prefix, long len);



    private:
        TLzTrie *trie = NULL;
};
}

#endif
