#!/bin/bash

buildfolder=build-folder
cliexec=lzt-cli

dictfolder=$1
dictfile=$2
inmem=$3
#testfile=lzt\_core/test\_files/smallDict.txt
testfile=$dictfolder/$dictfile
#testfile=test-dicts/german.txt
outdict=$dictfile.lzt
outfile=$dictfile.out.txt
numSequentialTests=3

rm $cliexec
cp $buildfolder/$cliexec .
echo "testing $cliexec on $testfile"
#chmod +x $cliexec
# test core interface
#echo "compression ..."
#time ./$cliexec c -i $testfile -d $outdict
echo "list all words ..."
time ./$cliexec l -d $outdict $inmem > $outfile
cmp $testfile $outfile
#test class interface
#echo "class interface ..."
#time ./$cliexec cls -i $testfile -d $outdict $inmem > $outfile
#cmp $testfile $outfile

#echo "sequential queries ..."
#./$cliexec sq -d $outdict -n $numSequentialTests $inmem

#rm -rf $outdict
#rm $outfile
