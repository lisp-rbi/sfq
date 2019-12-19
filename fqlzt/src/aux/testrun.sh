#!/bin/bash

#testfile=lzt\_core/test\_files/smallDict.txt
testfile=test-dicts/french.txt
#testfile=polishMorphDict.txt
outfile=testdict.txt

cp GNU-amd64-Linux/lzt-testrun.cpp testrun
chmod +x testrun
./testrun c -i $testfile -d testdict.lzt
./testrun l -d testdict.lzt > $outfile
cmp $testfile $outfile
