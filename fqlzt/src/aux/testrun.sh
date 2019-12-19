#!/bin/bash

#testfile=lzt\_core/test\_files/smallDict.txt
#testfile=test-dicts/french.txt
testfile=test-dicts/german.txt
outfile=testdict.txt
buildfolder=build-folder
cliexec=lzt-cli

rm $cliexec
cp $buildfolder/$cliexec .
#chmod +x $cliexec
# test core interface
./$cliexec c -i $testfile -d testdict.lzt
./$cliexec l -d testdict.lzt > $outfile
cmp $testfile $outfile
# test class interface
./$cliexec cls -i $testfile -d testdict.lzt > $outfile
cmp $testfile $outfile