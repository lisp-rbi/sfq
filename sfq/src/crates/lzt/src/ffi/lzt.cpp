#include "lzt.hpp"



Lzt::Lzt(string Path){
  // FXME: overload -> makie and read constructor in one!!
  read(Path);
}

Lzt::~Lzt(){
    //if (trie != NULL) freeTrieMem(trie);
}

bool Lzt::make(TSymbol* words, long length, string savePath, bool sortWords) {
    return createTrie(words, length, savePath, sortWords);
}

bool Lzt::read(string triePath) {
    trie = loadTrie(triePath);
    return trie != NULL;
}

vector<vector<TSymbol> >* Lzt::getRecords(vector<TSymbol> prefix) {
    return queryTrie(trie, prefix);
}


/* ABI:
 * Aplicationbinting interface for Rust fqlzt library
 */

extern "C" {

// ABI -> create an lzt object but do not keep it ...
    bool make_lzt( uchar* words, unsigned long wln, uchar* path, int pln) {

// Debug
/*
      cout <<  wln << " Start" << endl;
      for (int i = 0; i < (int) wln; i++)
      {

        if (words[i] == '\n'){
          cout << "not formated properly!!!\n";
          break;
        }
          (words[i] == zeroSymbol<uchar>()) ?
          (cout << endl) :
          (cout << words[i]);
      }
      (words[wln] == zeroSymbol<uchar>()) ?
      (cout <<".  ."<< (unsigned int) words[wln-1] <<".  ."<< (unsigned int) words[wln] <<".  ."<< (unsigned int) words[wln+1]<<".  ."<< (unsigned int) words[wln+2] <<  "zero") :
      (cout <<".  ."<< (unsigned int) words[wln-1]<<".  ."<< (unsigned int) words[wln] <<".  ."<< (unsigned int) words[wln+1]<<".  ."<< (unsigned int) words[wln+2]<<"not ");
      cout <<"\nStop" << endl;
*/
// Debug

      // Get output path
      std::string oPath(reinterpret_cast<char*>(path),pln);
      return Lzt::make(words, (long) wln, oPath, false);

    }


// ABI -> create an lzt object and keep it ...
    Lzt* open_lzt( uchar* path, int pln){
      std::string inPath(reinterpret_cast<char*>(path),pln);

      //cout << inPath << end;
      return new Lzt(inPath);
    }

// ABI -> manually delete an lzt object -> destruct
    void delete_lzt(Lzt *obj) {
        delete obj;
    }

// ABI -> query lzt : prefix search
		unsigned long query_lzt (Lzt *obj, uchar* pattern, unsigned long pln){

      vector<uchar> ptt(pattern, pattern + pln);
      vector<vector<uchar>>* out = obj->getRecords(ptt);

      obj->objvec = std::accumulate(
        out->begin(), out->end(), vector<uchar>(), [](vector<uchar> (a), vector<uchar> (b)) {
          a.insert(a.end(), b.begin(), b.end());
          a.push_back('\n');
          return a;
        }
      );

      return (unsigned long) obj->objvec.size();
		}

// ABI -> get query results
    void get_query_results (Lzt *obj, uchar* results){
      for (unsigned long i =0; i< obj->objvec.size(); i++){
        results[i] = obj->objvec[i];
      }
    }

}
