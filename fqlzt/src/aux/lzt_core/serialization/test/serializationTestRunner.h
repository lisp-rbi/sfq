#ifndef SERIALIZATIONTESTRUNNER_H
#define SERIALIZATIONTESTRUNNER_H

#include <cstdlib>
#include <iostream>

#include "CharArrayTests.h"
#include "../MemCharArray.h"
#include "../DiskCharArray.h"
#include "BitSequenceArrayTest.h"
#include "../BitSequenceArray.h"

using namespace std;

void memArrayTests();
void diskArrayTests();
void bitSeqArrayMemTests();
void bitSeqArrayDiskTests();

int runSerializationTests();

#endif /* SERIALIZATIONTESTRUNNER_H */

