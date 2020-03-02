#!/bin/bash

buildfolder=build-folder
cliexec=lzt-cli-old

dictfolder=$1
dictfile=$2
#testfile=lzt\_core/test\_files/smallDict.txt
testfile=$dictfolder/$dictfile
#testfile=test-dicts/german.txt
outdict=$dictfile.old.lzt
outfile=$dictfile.old.out.txt
numSequentialTests=4

rm $cliexec
cp $buildfolder/$cliexec .
echo "testing $cliexec on $testfile"
#chmod +x $cliexec
# test core interface
echo "compression ..."
time ./$cliexec c -i $testfile -d $outdict
echo "list all words ..."
time ./$cliexec l -d $outdict > $outfile
cmp $testfile $outfile
# test class interface
#echo "class interface ..."
#./$cliexec cls -i $testfile -d $outdict > $outfile
#cmp $testfile $outfile

echo "sequential queries ..."
./$cliexec sq -d $outdict -n $numSequentialTests

rm -rf $outdict
rm $outfile