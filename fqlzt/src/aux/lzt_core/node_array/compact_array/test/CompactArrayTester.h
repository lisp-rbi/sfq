#ifndef COMPACTARRAYTESTER_H
#define	COMPACTARRAYTESTER_H

#include <string>
#include <vector>
#include <sstream>
#include <iostream>
#include <fstream>

#include "../CompactArray.h"
#include "../CompactArrayBuilder.h"

#include "util/factory.h"
#include "util/Timer.h"
#include "util/TempFile.h"
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
    void testCreate(string dictset);
    void testSerialize(string dictset, bool toFolder);
    void testSerializeInPlace(string dictset);
 
    typedef VectorArray<TSymbol, TIndex> TMemNodeArray;
    
private:
         
    /** for a label defining a set of dictionaries, return a list 
     * of (dict name, dict path) pairs. */
    vector<pair<string,string> > dictSet(string label);
    /** load dictionaries as returned by dictset() method, in the exact same order. */
    vector<WordList<TSymbol>*> loadDictionaries(string label);
    // free dictionaries loaded by loadDictionaries
    void freeDictMem(vector<WordList<TSymbol>*> dicts);
    
    // test equality by directly comparing nodes stored in two node arrays
    template<typename TNa1, typename TNa2>
    void nodeArraysNodeEquality(const TNa1& na1, const TNa2& na2);
    // test equality by listing and comparing the words in array-based tries
    template<typename TNa1, typename TNa2>
    void nodeArraysTrieEquality(const TNa1& na1, const TNa2& na2);

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
    else if (label == "natural-lang") {
        subfolder = "natural-lang"; int numd = 2;
        char const * d[] = { "french.txt", "german.txt"};
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
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::testCreate(string dictset) {
    cout<<"testCreate, dictset="<<dictset<<endl;
    vector<pair<string,string> > dlabels = dictSet(dictset);
    vector<WordList<TSymbol>*> dicts = loadDictionaries(dictset);
    for (int i = 0; i < dicts.size(); ++i) {
        CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray> builder; 
        CompactArray<TSymbol, TIndex, TBitSequenceArray>* carray;
        carray = builder.createCompactArray(dicts[i], dlabels[i].first, false);
        TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*dicts[i]);
        nodeArraysNodeEquality(*carray, *array);
        nodeArraysTrieEquality(*carray, *array);
        delete array;
        delete carray;
    }
    freeDictMem(dicts);
    cout<<"testBuildSave, dictset="<<dictset<<" PASSED"<<endl;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::testSerialize(string dictset, bool toFolder) {
    // TODO ? extract serialization testing in a separate module, for generic ISerializable objects        
    cout<<"COMPACT ARRAY testSerialize(dictset="<<dictset<<") "<<"toFolder: "<<toFolder<<endl;    
    vector<pair<string,string> > dlabels = dictSet(dictset);
    vector<WordList<TSymbol>*> dicts = loadDictionaries(dictset);
    for (int i = 0; i < dicts.size(); ++i) {
        CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray> builder; 
        CompactArray<TSymbol, TIndex, TBitSequenceArray>* carray;
        carray = builder.createCompactArray(dicts[i], dlabels[i].first, false);            
        // serialize
        TempFile file(toFolder);    
        carray->persist(file.getName());          
        // deserialize
        CompactArray<TSymbol, TIndex, TBitSequenceArray> deserArray;
        deserArray.load(file.getName());        
        nodeArraysNodeEquality(*carray, deserArray);
        //nodeArraysTrieEquality(*carray, deserArray);
        //delete deserArray;
        delete carray;
    }
    freeDictMem(dicts);
    cout<<"COMPACT ARRAY testSerialize PASSED"<<endl;       
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::testSerializeInPlace(string dictset) {
    cout<<"COMPACT ARRAY testSerializeInPlace(dictset="<<dictset<<")"<<endl;    
    vector<pair<string,string> > dlabels = dictSet(dictset);
    vector<WordList<TSymbol>*> dicts = loadDictionaries(dictset);
    for (int i = 0; i < dicts.size(); ++i) {        
        // create uncompressed array
        TMemNodeArray* array = getLzArrayLCT<TMemNodeArray>(*dicts[i]);                
        // build and persist in-place
        CompactArrayBuilder<TSymbol, TIndex, TBitSequenceArray> builder;         
        TempFile file(true);    
        builder.buildSaveCompactArray(dicts[i], file.getName(), dlabels[i].first, false);        
        // load and compare                        
        CompactArray<TSymbol, TIndex, TBitSequenceArray> deserArray;
        deserArray.load(file.getName());        
        nodeArraysNodeEquality(*array, deserArray);
        //nodeArraysTrieEquality(*array, deserArray);        
        delete array;
    }
    freeDictMem(dicts);
    cout<<"COMPACT ARRAY testSerialize PASSED"<<endl;             
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TNa1, typename TNa2> void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::
nodeArraysNodeEquality(const TNa1& na1, const TNa2& na2) {
    cout<<"testing nodeArraysNodeEquality: ";
    TEST_ASSERT(na1.getSize() == na2.getSize());
    TEST_ASSERT(na1.isEnumerated() == na2.isEnumerated());
    Timer timer; timer.start();
    for (size_t i = 0; i < na1.getSize(); ++i) {
        typename TNa1::NodeConst n1 = na1[i];
        typename TNa2::NodeConst n2 = na2[i];
        TEST_ASSERT(n1.getSymbol() == n2.getSymbol());
        TEST_ASSERT(n1.getSibling() == n2.getSibling());
        if (na1.isEnumerated())
            TEST_ASSERT(n1.getNumWords() == n2.getNumWords());
        TEST_ASSERT(n1.getEow() == n2.getEow());
        TEST_ASSERT(n1.getCow() == n2.getCow());
        TEST_ASSERT(n1.isPointer() == n2.isPointer());
    }
    cout<<timer.elSeconds()<<" seconds elapsed"<<endl;
}

template <typename TSymbol, typename TIndex, typename TBitSequenceArray>
template<typename TNa1, typename TNa2> void CompactArrayTester<TSymbol, TIndex, TBitSequenceArray>::
nodeArraysTrieEquality(const TNa1& na1, const TNa2& na2) {
    cout<<"testing nodeArraysTrieEquality: ";
    // create tries, no destructors will be called to avoid deleting node arrays
    LzTrie<TNa1>* trie1 = new LzTrie<TNa1>(na1);
    LzTrie<TNa2>* trie2 = new LzTrie<TNa2>(na2); 
    Timer timer; timer.start();
    WordList<TSymbol>* words1 = trie1->getWordsByPrefix("");    
    cout<<timer.elSeconds()<<" seconds 4 trie1.listall; ";
    timer.start();
    WordList<TSymbol>* words2 = trie2->getWordsByPrefix("");    
    cout<<timer.elSeconds()<<" seconds 4 trie2.listall"<<endl;
    TEST_ASSERT(*words1 == *words2);    
    delete words1; delete words2;    
}


#endif	/* COMPACTARRAYTESTER_H */

