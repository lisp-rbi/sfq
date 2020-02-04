#ifndef CHARARRAYTESTS_H
#define CHARARRAYTESTS_H

#include <iostream>
#include <cassert>

using namespace std;

template <typename TCharArray>
class CharArrayTests {
public:
    CharArrayTests();    
    void basicInterfaceTest();
    virtual ~CharArrayTests();
private:

};

template <typename TCharArray>
CharArrayTests<TCharArray>::CharArrayTests() {
}

template <typename TCharArray>
void CharArrayTests<TCharArray>::basicInterfaceTest() { 
    TCharArray carray; // default constructor must exist
    cout<<"created object"<<endl;
    //carray.allocate(100); cout<<"allocate"<<endl;
    //carray[50] = 'a'; cout<<"assign"<<endl;
    //carray.resize(200); cout<<"resize"<<endl;
    //assert(carray[50] == 'a');
    //carray.freeMemory(); cout<<"free"<<endl;
}

template <typename TCharArray>
CharArrayTests<TCharArray>::~CharArrayTests() {
}


#endif /* CHARARRAYTESTS_H */

