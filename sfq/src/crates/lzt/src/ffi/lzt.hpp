#ifndef LZT_HPP
#define LZT_HPP

#include "lzt-interface.h"
// RB
#include "lzt-utils.h"
#include "lzt_core/util/regex.h"
//


#include <iostream>
#include <cstddef>

// RB
#include <vector>
#include <iterator>
#include <algorithm>
#include <numeric>
//


using namespace std;

class Lzt {

    public:

     /** RB:
      * FXME: move to private/protected
      */
      vector < TSymbol> objvec;


      /* Constructor! */
      Lzt(string Path);

      /* Destructor */
      ~Lzt();

      /**
       * Creates compressed trie from a sorted list of words
       * assumes dict does not exist and overwrites it
       */

       //static bool make(vector<vector<TSymbol> >* words, string savePath, bool sortWords=false);
       static bool make(TSymbol* words, long length, string savePath, bool sortWords);

       /**
         *  If trie exists, load the data structure into self
         *  Return true if loading is successful, else false.
         */

       bool read(string triePath);

        /**
         * Query (loaded) trie by prefix, return a list of all words sharing the prefix.
         * Empty prefix lists all words.
         */
       vector<vector<TSymbol> > * getRecords(vector<TSymbol> prefix);

     private:
        TLzTrieDisk *trie = NULL;
};


#endif
