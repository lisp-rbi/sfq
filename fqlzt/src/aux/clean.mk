#### Clean target deletes all generated files ####
clean:
	rm -f \
		$(TARGETDIR_lzt-testrun.cpp)/lzt-testrun.cpp \
		$(TARGETDIR_lzt-testrun.cpp)/lzt-interface.o \
		$(TARGETDIR_lzt-testrun.cpp)/lzt-testrun.o \
		$(TARGETDIR_lzt-testrun.cpp)/na_utils.o \
		$(TARGETDIR_lzt-testrun.cpp)/CompactArrayTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/CmmExporter.o \
		$(TARGETDIR_lzt-testrun.cpp)/CmmExporterTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/NodeArrayBrowser.o \
		$(TARGETDIR_lzt-testrun.cpp)/char_symbol.o \
		$(TARGETDIR_lzt-testrun.cpp)/instantiator.o \
		$(TARGETDIR_lzt-testrun.cpp)/EnumArrayTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/NodeArrayTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/CompressorTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/SuffixStructTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/TempFile.o \
		$(TARGETDIR_lzt-testrun.cpp)/etimer.o \
		$(TARGETDIR_lzt-testrun.cpp)/Timer.o \
		$(TARGETDIR_lzt-testrun.cpp)/utils.o \
		$(TARGETDIR_lzt-testrun.cpp)/FileReaderTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/TestCaseReaderTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/synth.o \
		$(TARGETDIR_lzt-testrun.cpp)/nstr.o \
		$(TARGETDIR_lzt-testrun.cpp)/spell.o \
		$(TARGETDIR_lzt-testrun.cpp)/hash.o \
		$(TARGETDIR_lzt-testrun.cpp)/nindex.o \
		$(TARGETDIR_lzt-testrun.cpp)/prefix.o \
		$(TARGETDIR_lzt-testrun.cpp)/buildu_fsa.o \
		$(TARGETDIR_lzt-testrun.cpp)/one_word_io.o \
		$(TARGETDIR_lzt-testrun.cpp)/common.o \
		$(TARGETDIR_lzt-testrun.cpp)/build_fsa.o \
		$(TARGETDIR_lzt-testrun.cpp)/guess.o \
		$(TARGETDIR_lzt-testrun.cpp)/unode.o \
		$(TARGETDIR_lzt-testrun.cpp)/morph.o \
		$(TARGETDIR_lzt-testrun.cpp)/nnode.o \
		$(TARGETDIR_lzt-testrun.cpp)/visualize.o \
		$(TARGETDIR_lzt-testrun.cpp)/accent.o \
		$(TARGETDIR_lzt-testrun.cpp)/HuffmanIndexMapTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/LzTrieDictTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/HuffmanCodecCreator.o \
		$(TARGETDIR_lzt-testrun.cpp)/HuffmanCoder.o \
		$(TARGETDIR_lzt-testrun.cpp)/HuffmanDecoder.o \
		$(TARGETDIR_lzt-testrun.cpp)/HuffmanTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/WordListTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/LzTrieIteratorTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/LzTrieTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/WordIndexerTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/TrieTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitPointer.o \
		$(TARGETDIR_lzt-testrun.cpp)/SerializationUtils.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitSequenceArray.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitSequence.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitVector.o \
		$(TARGETDIR_lzt-testrun.cpp)/serialization.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitSequenceArraySer.o \
		$(TARGETDIR_lzt-testrun.cpp)/CompactArraySerTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitSequenceTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/IntBitArrayTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitVectorTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/SerializationTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/BitSequenceArrayTest.o \
		$(TARGETDIR_lzt-testrun.cpp)/DebugException.o \
		$(TARGETDIR_lzt-testrun.cpp)/TestException.o \
		$(TARGETDIR_lzt-testrun.cpp)/StackTrace.o \
		$(TARGETDIR_lzt-testrun.cpp)/CharStringSA.o
	$(CCADMIN)
	rm -f -r $(TARGETDIR_lzt-testrun.cpp)

