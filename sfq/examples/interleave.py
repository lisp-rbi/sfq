import os
import sys
import math
import datetime as dt
import time
import string
import argparse as ap

parser = ap.ArgumentParser(description='Script that interleaves the forward and reverse fastq files',
                           epilog='''Thank you for using it! 
                                     Dalibor Hršak''',
                           usage='%(prog)s -i <fwd_file.fq> -j <rev_file.fq> -o <interleaved.fq>',
                           fromfile_prefix_chars='@')

parser.add_argument('-i', dest='fwd_file', nargs='+', metavar='FWD_FILE', default=[],
                    help='''Specify the name of the input forward fq file''')

parser.add_argument('-j', dest='rev_file', nargs='+', metavar='REV_FILE', default=[],
                    help='''Specify the name of the input reverse fq file''')

parser.add_argument('-o', dest='outputfile', nargs='+', metavar='OUTPUT_FILE', default=[],
                    help='''Specify the name of the interleaved output file.''')

if len(sys.argv) == 1:
    parser.print_help()
    sys.exit()

args = parser.parse_args()
fwd_file=args.fwd_file[0]
rev_file=args.rev_file[0]
outputfile=args.outputfile[0]

print("I will read the files {0} and {1} and I will\ninterleave them and write them into the file {2}.".format(fwd_file, rev_file, outputfile))

with open(outputfile, 'w') as outfile:
    with open(fwd_file, 'r') as f1, open(rev_file, 'r') as f2:
        tmp_fwd = list()
        tmp_rev = list()
        for fwd_line, rev_line in zip(f1, f2):
            if (len(tmp_fwd) == 0) and (len(tmp_rev) == 0):
                x = fwd_line.strip() + "F"
                y = rev_line.strip() + "R"
            else:
                x = fwd_line.strip() 
                y = rev_line.strip() 
            tmp_fwd.append(x)
            tmp_rev.append(y)
            if (len(tmp_fwd) == 4) and (len(tmp_rev) == 4):
                for x in tmp_fwd:
                    outfile.write("{0}\n".format(x))
                for y in tmp_rev:
                    outfile.write("{0}\n".format(y))
                tmp_fwd = list()
                tmp_rev = list()
