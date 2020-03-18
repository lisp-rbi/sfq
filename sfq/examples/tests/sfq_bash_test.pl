#!/usr/bin/perl


use strict;
use warnings;
use Capture::Tiny ':all';

## Designed specifically for sfq!!
# external command

my $cmd = "sfq";
my $fwd = "in_r1.fq";
my $rev = "in_r2.fq";
my $out = "my_out";
# 1 - print only mode , 0 - test mode
my $print = 0;


##  command option set

my @options =(
["-i "],
["-o "],
["-j "],
["-a "],
["-t "],
["-f "],
["-l "],
["-M "],
["-m "],
);


my @optionSet = (
# -i
["$fwd.fa","$fwd.fq"],
# -o
["$out.fa","$out.fq"],
# -j
["$rev.fa", "$rev.fq"],
# -a
["c","d","g"],
# -t
["fasta", "fastq"],
# -f
["Fa", "Fq", "\"h\"", "\"s\"", "\"q\"", "\"h+s\"", "\"s+h\"", "\"h+s+q\"", "\"q+s+h\"", "\"s+s\""],
# -l
["\"rand(7)\"", "list.list"],
# -M
["1000", "10000", "Max"],
# -m
["D", "R"]
);


# FWD/REV Compress
# -i [] -o [-j] -t [2] -a c -M [3]

  for (my $i = 0; $i < @{$optionSet[7]}; $i++ ){
    for (my $j = 0; $j< @{$optionSet[4]}; $j++ ){
      for (my $l = 0; $l < @{$optionSet[8]}; $l++){
        my $exe_1 =  "$cmd $options[0][0] $optionSet[0][$j] $options[1][0] $optionSet[1][$j] $options[3][0] $optionSet[3][0] $options[7][0] $optionSet[7][$i] $options[4][0] $optionSet[4][$j] $options[8][0] $optionSet[8][$l]";
        my $exe_2  = "$cmd $options[0][0] $optionSet[0][$j] $options[1][0] $optionSet[1][$j] $options[2][0] $optionSet[2][$j] $options[3][0] $optionSet[3][0] $options[7][0] $optionSet[7][$i] $options[4][0] $optionSet[4][$j] $options[8][0] $optionSet[8][$l]";
        &execute($exe_1,$print);
        &execute($exe_2,$print);
      }
    }
  }

# Decompress
# -i -o -t [2] -a d  -f [11] -m [2]
# Grep
# -i -o -t [2] -a g  -f [11] -m [2] -l [2]

for (my $i = 0; $i < @{$optionSet[8]}; $i++ ){
  for (my $j = 0; $j< @{$optionSet[4]}; $j++ ){
    for (my $l = 0; $l < @{$optionSet[5]}; $l++){
      my $exe_1 =  "$cmd $options[0][0] $optionSet[1][$j] $options[1][0] $optionSet[1][$j].interl $options[3][0] $optionSet[3][1] $options[4][0] $optionSet[4][$j] $options[5][0] $optionSet[5][$l] $options[8][0] $optionSet[8][$i]";
      &execute($exe_1,$print);
      for (my $k = 0; $k < @{$optionSet[8]}; $k++){
        my $exe_2  = "$cmd $options[0][0] $optionSet[1][$j] $options[1][0] $optionSet[1][$j].interl $options[3][0] $optionSet[3][2] $options[4][0] $optionSet[4][$j] $options[5][0] $optionSet[5][$l] $options[8][0] $optionSet[8][$i] $options[6][0] $optionSet[6][$k]";
        &execute($exe_2, $print);
      }
    }
  }
}


sub execute {

    my ($cmd, $print_only) = @_;

    print "Testing: $cmd  ... \n";

    if (!$print_only){

      my ($stdout, $stderr, $exit) = capture {
        system( $cmd);
      };

      if ($exit eq 0){
        print "ok!\n";
      }else{
        print "FAILED!!\n Error: \n $stdout \n $stderr \n";
      }
    }
}
