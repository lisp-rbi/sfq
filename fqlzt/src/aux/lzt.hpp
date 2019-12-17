#ifndef LZT_HPP
#define LZT_HPP



class Lzt {

   void *internal;

public:
   /* Constructor/Desctructor */
	Lzt();
	~Lzt();

   /* Functions */
  void test_binding (int64_t x);


//	vector<vector<char>>& get_records(vector<vector<char>>& prefix);
//  bool load();
//  bool make();


};


///////////////////////////////////////////////////////////////////////////////
//
//   For now just implement functions here and test them seperatly using plain
//   c++. Binding is not working and I have no more time today to fix it ...
//   Sorry..  RB
//
///////////////////////////////////////////////////////////////////////////////

/*  Constructor  */

Lzt::Lzt(){
  cout << "Constructor works" << endl;
}

/* Functions */

// Implement functions here

void Lzt::test_binding (int64_t x){
   return x+x;
}


/* Getters */

vector<vector<char>>& Lzt::get_records(vector<vector<char>>& prefix){

  vector<vector<int>> vec(prefix.size(), vector<char>(prefix.size(),'x'));
  // lzt magic happens here ..................
  return vec

}


/* Setters */

#endif
