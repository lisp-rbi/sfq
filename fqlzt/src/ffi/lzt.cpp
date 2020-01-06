#include "lzt.hpp"


// Maybe a better solution?
//Lzt::Lzt(vector<vector<TSymbol> >* words, string savePath){
//  return make(words,savePath);
//}

Lzt::Lzt(string Path){
  // FXME: overload -> making and reading constructor in one!!
  read(Path);
}

Lzt::~Lzt(){
    if (trie != NULL) freeTrieMemory(trie);
}

// FXME: This needs to be re-written. Only simple structs are allowed
bool Lzt::make(vector<vector<TSymbol> >* words, string savePath) {
    return createLzTrie(words, savePath);
}

bool Lzt::read(string triePath) {
    trie = loadLzTrie(triePath);
    return trie != NULL;
}

// FXME: This needs to be re-written. Only simple structs are allowed
vector<vector<TSymbol> >* Lzt::getFastqRecords(vector<TSymbol> prefix) {
  if(trie != NULL){cout <<"problem !!\n";}
    return queryLzTrie(trie, prefix);
}


// ABI
extern "C" {
// ABI -> create an lzt object but do not keep it ...
    bool make_lzt( uchar* words, unsigned long wln, uchar* path, int pln) {

      unsigned long l=0, x=0;
      // Get output path
      std::string oPath(reinterpret_cast<char*>(path),pln);

      // Compute word length -> FXME: this coule be done better by passing word length
      for (unsigned long i = 0; i < wln; i++){
        if ( words[i] == '\n' ){break;}
        l++;
      }
      // Make 2d vec -> N= x(l+1)
      vector<vector<unsigned char> > vec2d((wln+1)/(l+1) , vector<unsigned char> (l));
      for (unsigned long i = 0; i < wln+1; i+=(l+1)){
        for(unsigned long j = 0; j < l; j++){
          vec2d[x][j] = words[i+j];
        }
        x++;
      }
      return Lzt::make(&vec2d, oPath);
    }


// ABI -> create an lzt object and keep it ...


    Lzt* open_lzt( uchar* path, int pln){
      std::string inPath(reinterpret_cast<char*>(path),pln);
      return new Lzt(inPath);
    }

// ABI -> manually delete an lzt object -> destruct

    void delete_lzt(Lzt *obj) {
        delete obj;
    }

		void query_lzt (Lzt *obj, uchar* pattern, unsigned long pln, uchar* res, unsigned long rln){

      vector<uchar> ptt(pattern, pattern + pln);

      for (unsigned long i =0; i< pln; i++){
        cout << ptt[i] << "\n";
      }

      vector<vector<uchar>>* out = obj->getFastqRecords(ptt);

      vector<uchar> vec1d = std::accumulate(
        out->begin(), out->end(), vector<uchar>(), [](vector<uchar> (a), vector<uchar> (b)) {
          a.insert(a.end(), b.begin(), b.end());
          a.push_back('\n');
          return a;
        }
      );
/*      vector<char> vec1d(
        reinterpret_cast<char*>(uvec1d.data()),
        reinterpret_cast<char*>(uvec1d.data()) + uvec1d.size()
      );

      vector<uchar>().swap(uvec1d);
*/
      for (unsigned long i =0; i< vec1d.size(); i++){
        cout << vec1d[i] << "\n";
      }
      res = &vec1d[0];
      rln = vec1d.size();

		}

}




/*
class Obj {

public:
	Obj(vector<int> vec);
	~Obj();
  int get(int x);

private:

vector<int> pvec;

};


Obj::Obj(vector<int> vec){
  pvec = vec;
}

Obj::~Obj(){
   vector<int>().swap(pvec);
}


int Obj::get(int x) {
  if (x < (int) pvec.size()){
    return (int) pvec[x];
  }else{
    return (int)  -1;
  }

}


extern "C" {
    Obj* create_object(int* items, int len) {
         vector<int> vec(len);
				 for (int i = 0; i< len; i++){
					 vec[i] = items[i];
				 }
         return new Obj(vec);
    }
    void delete_object(Obj *obj) {
        delete obj;
    }

		int get_data (Obj *obj, int x){
			return (*obj).get(x);
		}
}





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

      return true;
  }



}

*/
