extern crate cc;

// Compile cc library located in ffi
fn main() {

        println!("cargo:rustc-link-lib=lzt");


        cc::Build::new()
            .file("src/ffi/lzt.cpp")
            .file("src/ffi/lzt-interface.cpp")
            .file("src/ffi/lzt-utils.cpp")
            .file("src/ffi/lzt_core/node_array/na_utils.cpp")
            .file("src/ffi/lzt_core/node_array/util/CmmExporter.cpp")
            .file("src/ffi/lzt_core/node_array/na_browser/NodeArrayBrowser.cpp")
            .file("src/ffi/lzt_core/node_array/types/symbol/char/char_symbol.cpp")
            .file("src/ffi/lzt_core/node_array/concepts/instantiator.cpp")
            .file("src/ffi/lzt_core/util/TempFile.cpp")
            .file("src/ffi/lzt_core/util/etimer.cpp")
            .file("src/ffi/lzt_core/util/Timer.cpp")
            .file("src/ffi/lzt_core/util/utils.cpp")
            .file("src/ffi/lzt_core/dictionary/lztrie_dict/huffman/HuffmanCodecCreator.cpp")
            .file("src/ffi/lzt_core/dictionary/lztrie_dict/huffman/HuffmanCoder.cpp")
            .file("src/ffi/lzt_core/dictionary/lztrie_dict/huffman/HuffmanDecoder.cpp")
            .file("src/ffi/lzt_core/serialization/BitPointer.cpp")
            .file("src/ffi/lzt_core/serialization/SerializationUtils.cpp")
            .file("src/ffi/lzt_core/serialization/BitSequenceArray.cpp")
            .file("src/ffi/lzt_core/serialization/BitSequence.cpp")
            .file("src/ffi/lzt_core/serialization/BitVector.cpp")
            .file("src/ffi/lzt_core/serialization/serialization.cpp")
            .file("src/ffi/lzt_core/serialization/array/BitSequenceArraySer.cpp")
            .file("src/ffi/lzt_core/debug/DebugException.cpp")
            .file("src/ffi/lzt_core/debug/StackTrace.cpp")
            .file("src/ffi/lzt_core/suffix_array/CharStringSA.cpp")
            .cpp(true)
            .cpp_link_stdlib("stdc++")
            .compile("liblzt.a");
}
