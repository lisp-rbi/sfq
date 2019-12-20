#!/bin/bash

buildfolder=build-folder
cliexec=lzt-cli

dictfolder=$1
dictfile=$2
#testfile=lzt\_core/test\_files/smallDict.txt
testfile=$dictfolder/$dictfile
#testfile=test-dicts/german.txt
outdict=$dictfile.lzt
outfile=testdict.txt

rm $cliexec
cp $buildfolder/$cliexec .
echo "testing $cliexec on $testfile"
#chmod +x $cliexec
# test core interface
echo "compression ..."
./$cliexec c -i $testfile -d $outdict
echo "list all words ..."
./$cliexec l -d $outdict > $outfile
cmp $testfile $outfile
# test class interface
echo "class interface ..."
./$cliexec cls -i $testfile -d $outdict > $outfile
cmp $testfile $outfile

rm $outfile
