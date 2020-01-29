//#include "lzt.hpp"
#include "lzt-interface.h"
#include "lzt-utils.h"
#include "lzt_core/util/regex.h"

#include <iostream>
#include <cstddef>
extern "C" {



static void printWordList(string query, TLzTrie* lzTrie);



void printWordList(string query, TLzTrie* lzTrie) {
    // convery string of chars to string of TSymbols
    TSymbol *queryTS = stringToTSymbolString(query);
    //cout<<"start printing"<<endl;
    if (query.find('*') != string::npos) { // list words
        TSymbol prefix[1000];
        bool syntaxOk = getPrefixBeforeStar<TSymbol>(queryTS, prefix);
        //cout<<"prefix calculated"<<endl;
        if (syntaxOk) {
            WordList<TSymbol>* list = lzTrie->getWordsByPrefix(prefix);
            wordListToStreamChars<TSymbol>(list, cout);
            //cout<<"number of words: "<<list->numberOfWords()<<endl;
            delete list;
        }
        else {
            cout<<"* symbol must be the last symbol of the input."<<endl;
        }
    }
    else { // print a single word
        bool contains = lzTrie->containsWord(queryTS);
        if (contains) cout<<"in the dictionary"<<endl;
        else cout<<"NOT in the dictionary"<<endl;
    }

    delete [] queryTS;
}


/*
  Lzt::Lzt(){
  }

  Lzt::~Lzt(){
      if (trie != NULL) freeTrieMemory(trie);
  }
*/
  bool make(uchar* words, long len, char* path){
    std::string oPath(reinterpret_cast<char*>(path));
//    cout <<">"<< words[1] << " " << len << " " <<oPath <<  endl;
    long l=0, x=0;

    for (long i = 0; i < len; i++){
      if ( words[i] == '\n' ){break;}
      l++;
    }

    // N= x(l+1)
    vector<vector<unsigned char> > vec2d(len/(l+1) , vector<unsigned char> (l));
    for (long i = 0; i < len; i+=(l+1)){
      for(long j = 0; j < l; j++){
        vec2d[x][j] = words[i+j];
        cout<<vec2d[x][j]<<"";
      }
      cout<<endl;
      x++;
    }
    return createLzTrie(&vec2d, oPath);
  }

  bool read(uchar* path) {
    std::string tPath(reinterpret_cast<char*>(path));

    cout<<"her" << tPath <<".."<<endl;

    TLzTrie* trie = loadLzTrie(tPath);
    string query = "*";
    printWordList(query, trie);
      cout<<"her" << tPath <<".."<<endl;
//      freeTrieMemory(trie);
    return true; // trie != NULL;
  }

  bool getFastqRecords(uchar* prefix, long len) {
      //convert uchar into vec<uchar>
      vector<uchar> pfx(&prefix[0], &prefix[len]);
//      vector<vector<uchar>>* vec2d = queryLzTrie(trie, pfx);
/*      for (long i = 0; i<(*vec2d).size(); i++){

      }*/
      return true;
  }


/*
  bool read(string Path){

  }

  char* getFastqRecords(char* prefix){

  }
*/
  uchar* getStr(uchar* prefix){
      return prefix;
  }

  bool some_c_function (int* vec, int x, int l){

    std::cout << "Vec size: " << vec[0]  << " " << x<<"  "<<l<< std::endl;

    for (int i =0; i < x; i++){
      if (i%l ==0 && i>0){
        cout<<endl;
      }
      cout << vec[i] << "";

    }

    return true;
  }




}

/*

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


  int testbind(int c) {
    return c+4;
  }
}
*/
