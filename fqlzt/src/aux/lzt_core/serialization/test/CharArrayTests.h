#ifndef CHARARRAYTESTS_H
#define CHARARRAYTESTS_H

#include <iostream>

using namespace std;

template <typename TCharArray>
class CharArrayTests {
public:
    CharArrayTests();
    void test1();
    virtual ~CharArrayTests();
private:

};

template <typename TCharArray>
CharArrayTests<TCharArray>::CharArrayTests() {
}

template <typename TCharArray>
void CharArrayTests<TCharArray>::test1() { 
    cout<<"char array test1"<<endl;
}

template <typename TCharArray>
CharArrayTests<TCharArray>::~CharArrayTests() {
}


#endif /* CHARARRAYTESTS_H */

