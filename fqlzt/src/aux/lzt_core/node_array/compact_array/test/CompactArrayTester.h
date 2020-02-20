#ifndef COMPACTARRAYTESTER_H
#define	COMPACTARRAYTESTER_H

#include <string>
#include <vector>
#include <sstream>
#include <iostream>
#include <fstream>

#include "../CompactArray.h"
#include "../CompactArrayBuilder.h"

#include "dictionary/char_trie/Trie.h"
#include "dictionary/lz_trie/LzTrie.h"
#include "compress/lz_compressor/LzCompressor.h"
#include "util/WordFileReader.h"
#include "dictionary/util/WordList.h"
#include "node_array/na_utils.h"
#include "debug/lzt_test.h"

using namespace std;

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
class CompactArrayTester {

public:
    
    void testWithDictionaries();
    void simpleTests();
    void testBuildSave(string dictset);
    
private:
 
    /** for a label defining a set of dictionaries, return a list 
     * of (dict name, dict path) pairs. */
    vector<pair<string,string> > dictSet(string label);
    /** load dictionaries as returned by dictset() method, in the exact same order. */
    vector<WordList<TSymbol>*> loadDictionaries(string label);
    // free dictionaries loaded by loadDictionaries
    void freeDictMem(vector<WordList<TSymbol>*> dicts);

};

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
vector<pair<string,string> >  CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::
dictSet(string label) {
    string dictFolder = "test-dicts", subfolder;
    vector<string> dicts;    
    if (label == "small-dicts") {
        subfolder = "small"; int numd = 4;
        char const * d[] = { "onelongword.txt", "smalldict.txt", "polmorph.txt", "filepaths.txt" };
        for (int i = 0; i < numd; ++i) dicts.push_back(d[i]);
    }     
    vector<pair<string,string> > res;
    for (int i = 0; i < dicts.size(); ++i) {
        string fname = dicts[i];
        string path = dictFolder+"/"+subfolder+"/"+fname;
        res.push_back(pair<string, string>(fname, path));
    }
    return res;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
vector<WordList<TSymbol>*>  CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::
loadDictionaries(string dictSubset) {    
    vector<pair<string,string> > dicts = dictSet(dictSubset);
    vector<WordList<TSymbol>*> res;
    for (vector<pair<string,string> >::iterator it = dicts.begin(); it != dicts.end(); ++it) {        
        WordFileReader<TSymbol> reader(it->second); reader.readWords();
        res.push_back(reader.getWords());   
    }
    return res;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::
freeDictMem(vector<WordList<TSymbol>*> dicts) {    
    for (int i = 0; i < dicts.size(); ++i) delete dicts[i];      
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::testBuildSave(string dictset) {
    cout<<"testBuildSave, dictset="<<dictset<<endl;
    vector<pair<string,string> > dlabels = dictSet(dictset);
    vector<WordList<TSymbol>*> dicts = loadDictionaries(dictset);
    for (int i = 0; i < dicts.size(); ++i) {
        CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray> builder;                
        builder.buildSaveCompactArray(dicts[i], "", dlabels[i].first, false);
    }
    freeDictMem(dicts);
    cout<<"testBuildSave, dictset="<<dictset<<" PASSED"<<endl;
}

/** Read a number of small arrays, each for some special case of compactification,
 * and verifiy they get compacted correctly. */
template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::simpleTests() {
//    WordFileReader<TSymbol> reader("../lzt_core/node_array/compact_array/test/stringArrays.txt");
//    reader.readWords();
//
//    for (size_t i = 0; i < reader.getNumberOfWords(); ++i) {
//        string strArray = reader.getWord(i);
//        TNodeArray* array = nodeArrayFromString<TNodeArray>(strArray);
//        CompactArrayCreatorL<TNodeArray> compactCreator(*array);
//        CompactArrayL<TSymbol, TIndex>* compactArray = compactCreator.createCompactArray();
//        string strCompArray = nodeArrayToString(*compactArray);
//
//        ostringstream message; message << "i: " << i << endl;
//        message << "uncompacted: " << strArray << endl;
//        message << "compacted: " << strCompArray << endl;
//
//        TEST_ASSERT_MESSAGE(strArray == strCompArray, message.str());
//
//        delete array;
//        delete compactArray;
//    }
}

/** Test CompactNodeArray creation and access by creating compressed TNodeArray,
 * compacting it and comparing two array node by node. Also test that LZTrie
 * based on compact array contains all the words.  */
template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::testWithDictionaries() {

//    int const numOfDicts = 1;
//    char const * dicts[] = { "../dictionary_files/test/smallDict.txt", "../dictionary_files/french.txt" };
//    
//    for (int i = 0; i < numOfDicts; ++i) {
//        ostringstream message;
//        message << "testWithDictionaries: " << dicts[i];
//
//        WordFileReader<TSymbol> reader(dicts[i]);
//        Trie<TNodeArray> trie;
//
//        reader.readWords();
//        for (size_t j = 0; j < reader.getNumberOfWords(); ++j) {
//            TSymbol const * word = reader.getWord(j);
//            trie.insertWord(word);
//        }
//
//        LzCompressor<TNodeArray> comp;
//        TNodeArray& nodes = trie.exportNodeArray();
//        comp.compressArray(nodes);        
//
//        CompactArrayCreatorL<TNodeArray> compactCreator(nodes);
//        CompactArrayL<TSymbol, TIndex>* compactArray = compactCreator.createCompactArray();
//
//        // test nodeArray and compact node Array equality
//        string nodesString = nodeArrayToString(nodes);
//        string compactString = nodeArrayToString(*compactArray);
//        TEST_ASSERT_MESSAGE(nodesString == compactString, message.str());
//
//        // LZTrie based on compact array must contain all words
//        LzTrie<CompactArrayL<TSymbol, TIndex> > lztrie(*compactArray);
//
//        for (size_t j = 0; j < reader.getNumberOfWords(); ++j) {
//            TSymbol const * word = reader.getWord(j);
//            //TODO ovdje bi trebalo rjesiti genericki TSymbol* -> string
//            ostringstream message2;
//            message2 << message << endl << "word index: " << j;
//
//            TEST_ASSERT_MESSAGE(lztrie.containsWord(word), message2.str() );
//        }
//    }

}


#endif	/* COMPACTARRAYTESTER_H */

