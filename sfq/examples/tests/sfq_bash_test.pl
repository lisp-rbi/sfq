#!/usr/bin/perl


use strict;
use warnings;
use Capture::Tiny ':all';

## Designed specifically for sfq!!
# external command

my $cmd = "sfq";
my $fwd = "In/n.R1";
my $rev = "In/n.R2";
my $out = "my_out";
# 1 - print only mode , 0 - test mode
my $print = ($ARGV[0]) ? ($ARGV[0]) : (0);


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
["-s "]
);


my @optionSet = (
# -i
["$fwd.fa","$fwd.fq"],
# -o
["$out.fa","$out.fq", "$out.lossy.fa","$out.lossy.fq",],
# -j
["$rev.fa", "$rev.fq"],
# -a
["c","d","g"],
# -t
["fasta", "fastq"],
# -f
["\"fa\"", "\"fq\"", "\"h\"", "\"s\"", "\"q\"", "\"h+s\"", "\"s+h\"", "\"h+s+q\"", "\"q+s+h\"", "\"s+s\""],
# -l
["\"rand(7)\"", "list.list"],
# -M
["1000", "10000", "Max"],
# -m
["D", "R"],
# -s
["complete", "lossy"]
);


#Test summary
my ($pass, $fail, $total) = (0,0,0);


for (my $n = 0; $n < @{$optionSet[9]}; $n++ ){

# FWD/REV Compress
# -i [] -o [-j] -t [2] -a c -M [3]

  for (my $i = 0; $i < @{$optionSet[7]}; $i++ ){
    for (my $j = 0; $j< @{$optionSet[4]}; $j++ ){
      for (my $l = 0; $l < @{$optionSet[8]}; $l++){
        my $c = ($n == 0) ? ($j+$n): ($j+$n+1);
        my $exe_1 =  "$cmd $options[0][0] $optionSet[0][$j] $options[1][0] $optionSet[1][$c] $options[3][0] $optionSet[3][0] $options[7][0] $optionSet[7][$i] $options[4][0] $optionSet[4][$j] $options[8][0] $optionSet[8][$l] $options[9][0] $optionSet[9][$n]";
        my $exe_2  = "$cmd $options[0][0] $optionSet[0][$j] $options[1][0] $optionSet[1][$c] $options[2][0] $optionSet[2][$j] $options[3][0] $optionSet[3][0] $options[7][0] $optionSet[7][$i] $options[4][0] $optionSet[4][$j] $options[8][0] $optionSet[8][$l] $options[9][0] $optionSet[9][$n]";
        my ($q,$w) = &execute($exe_1,$print);
        $pass+=$q; $fail+=$w;
        ($q,$w) =&execute($exe_2,$print);
        $pass+=$q; $fail+=$w;
        $total++;$total++;
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
        my $c = ($n == 0) ? ($j+$n): ($j+$n+1);
        my $exe_1 =  "$cmd $options[0][0] $optionSet[1][$c] $options[1][0] $optionSet[1][$c].interl $options[3][0] $optionSet[3][1] $options[4][0] $optionSet[4][$j] $options[5][0] $optionSet[5][$l] $options[8][0] $optionSet[8][$i] $options[9][0] $optionSet[9][$n]";
        my ($q,$w) = &execute($exe_1,$print);
        $pass+=$q; $fail+=$w;
        $total++;
        for (my $k = 0; $k < @{$optionSet[8]}; $k++){
          my $exe_2  = "$cmd $options[0][0] $optionSet[1][$c] $options[1][0] $optionSet[1][$c].interl $options[3][0] $optionSet[3][2] $options[4][0] $optionSet[4][$j] $options[5][0] $optionSet[5][$l] $options[8][0] $optionSet[8][$i] $options[6][0] $optionSet[6][$k] $options[9][0] $optionSet[9][$n]";
          my ($qq,$wq) = &execute($exe_2, $print);
          $pass+=$qq; $fail+=$wq;
          $total++;
        }
      }
    }
  }
}


print "In summary:\nPASS: $pass\nFAIL: $fail \nTOTAL: $total\n\n";

sub execute {

    my ($cmd, $print_only) = @_;

    print "Testing: $cmd  ... ";

    my ($ok,$nok) =(0,0);
    if (!$print_only){

      my ($stdout, $stderr, $exit) = capture {
        system( $cmd);
      };

      if ($exit eq 0){
        print "ok!\n";
        $ok = 1;
      }else{
        print "FAILED!!\n Error: \n $stdout \n $stderr \n";
        $nok = 1;
      }
    }

    return ($ok, $nok)

}
