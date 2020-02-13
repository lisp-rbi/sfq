#include <fstream>

#include "DiskCharArray.h"

const string DiskCharArray::PERSIST_FNAME = "DiskCharArray.bin";

/** Create file with the random name in . and bind to object.
 * Upon I/O fail, file == NULL, fname == "". */
DiskCharArray::DiskCharArray() { 
    numOfBlocks = 0;
    bindToRandomFile();
}

DiskArrayChar DiskCharArray::operator[](size_t i) {    
    return DiskArrayChar(this, i, readCharacter(i));
}

char DiskCharArray::readCharacter(size_t i) {    
    fseek(file, i, SEEK_SET);    
    char c;
    int res = fread((void*)&c, 1, 1, file);
    if (DEBUG) cout<<"readCharacter("<<i<<")"<<" c="<<int(c)<<" res="<<res<<endl;
    return c;
}

void DiskCharArray::writeCharacter(size_t i, char ch) {    
    fseek(file, i, SEEK_SET);
    //char *buffer = new char[1]; buffer[0] = ch;    
    //int res = fwrite((void*)buffer, 1, 1, file);
    int res = fwrite((void*)&ch, 1, 1, file);
    //delete buffer;
    if (DEBUG) cout<<"writeCharacter("<<i<<","<<int(ch)<<")"<<" res="<<res<<endl;    
}

/** Close the file pointed to by this.fname */
bool DiskCharArray::closeFile() {  
    if (state == STATE_CLOSED or state == STATE_ERROR) return;
    bool res = fclose(file) == 0;
    if (iobuffer != NULL) {
        delete [] iobuffer;
        iobuffer = NULL;
    }
    if (DEBUG) cout<<"closeFile()"<<" fname="<<fname<<" res="<<res<<endl;
    if (res) state = STATE_CLOSED;
    else state = STATE_ERROR;
    return res;
}

/** Open the file pointed to by this.fname. 
 * Create new file if such file does not exist.*/
bool DiskCharArray::openFile() {
    if (fileExists() == false) {
        if (createEmptyFile() == false) return false;        
    }
    file = fopen(fname.c_str(), "r+b");
    bool res = file != NULL;
    if (res) { // setup buffering
        if (BUFFER_SIZE > 0) {
            iobuffer = new char[BUFFER_SIZE];
            setvbuf(file, iobuffer, _IOFBF, BUFFER_SIZE);
        }
        else {
            iobuffer = NULL;
            setvbuf(file, NULL, _IONBF, 0); // no buffering
        }
    } else iobuffer = NULL;
    if (DEBUG) cout<<"openFile()"<<" fname="<<fname<<" res="<<res<<endl;
    return res;
}

/** Check if file with name this.fname exists. */
bool DiskCharArray::fileExists() {
    FILE *tfile;    
    tfile = fopen(fname.c_str(), "r");
    if (tfile != NULL) {        
        fclose(tfile);    
        return true;
    }
    else return false;    
}

/** Create empty file with name this.fname (overwrite existing!) */
bool DiskCharArray::createEmptyFile() {
    FILE *tfile;   
    tfile = fopen(fname.c_str(), "w");
    bool res = tfile != NULL;
    fclose(tfile);  
    return res;
}


/** Open the file pointed to by this.fname */
bool DiskCharArray::flushFile() {
    int res = fflush(file);
    return res == 0;
}

bool DiskCharArray::deleteFile() {    
    closeFile();    
    bool res = remove(fname.c_str()) == 0;
    if (DEBUG) cout<<"deleteFile(), res="<<res<<endl;
    return res;
}

/** Create file with a rnd filename, and use the file for this object's storage. */
void DiskCharArray::bindToRandomFile() {
    // TODO return value
    //string name = "/datafast/tmp/fastalzt/file_" + getRandomString(); // SSD
    string name = "file_" + getRandomString();
    name += ".tmp";
    fname = name;
    if (openFile() == false) fname = "";        
}

bool DiskCharArray::allocate(size_t size) {    
    fseek(file,0,SEEK_SET);
    char *buffer = (char *)malloc(size); //new char[size];    
    size_t numWritten = fwrite(buffer, 1, size, file);
    free(buffer); // delete [] buffer;
    if (DEBUG) cout<<"allocate("<<size<<")"<<" numWritten="<<numWritten<<endl;
    if (numWritten != size) return false;
    closeFile();    
    //if (flushFile()) {
    bool open=openFile();
    if (open) {
        if (DEBUG) cout<<" openFile()="<<open<<endl;
        numOfBlocks = size;
        return true;
    }
    else return false;    
}

bool DiskCharArray::resize(size_t size) {
    if (flushFile() == false) return false;
    if (size > numOfBlocks) { // write bytes to file's end
        fseek(file,0,SEEK_END);
        size_t diff = size-numOfBlocks;
        char *buffer = new char[diff];
        fwrite(buffer, 1, diff, file);
        delete [] buffer;
        int res = fflush(file);
        if (res == 0) numOfBlocks = size;
        return res == 0;
    }
    else if (size < numOfBlocks) { // truncate file, POSIX-only solution        
        int res = ftruncate(fileno(file), size);
        if (res == 0) numOfBlocks = size;
    }
    // check new size
    fseek(file, 0, SEEK_END);
    long newSize = ftell(file);
    assert(size == newSize);
}

void DiskCharArray::freeMemory() {
    // TODO empty fname handling
    if (DEBUG) cout<<"freeMemory()"<<endl;
    deleteFile();
    openFile();    
    numOfBlocks = 0;
}

bool DiskCharArray::persist(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {
            ofstream output(f.c_str());
            writeToStream(output);            
            output.close();    
            return output.good();
        }
        else if (file_is_directory(f)) {                
//            string fname = accessible_filename(f, PERSIST_FNAME);
//            if (fname == "") return false;
//            ofstream output(fname.c_str());
//            writeFieldsToStream(output);
//            output.close();
//            bool arrayWrite = charArray->persist(f);
//            return output.good() and arrayWrite;
        }
        else return false;
    }
    else return false;
}

bool DiskCharArray::load(string f) {
    if (file_accessible(f)) {
        if (file_is_regular(f)) {
            cout<<"load from: "<<f<<endl;
            fstream fstr(f.c_str());
            readFromStream(fstr);            
            fstr.close();    
            return fstr.good();
        }
        else if (file_is_directory(f)) {                

        }
        else return false;
    }
    else return false;
}

void DiskCharArray::writeToStream(ostream& stream) {
    if (DEBUG) cout<<"writeToStream, this.fname: "<<fname<<endl;
    // write persistable fields to stream
    SerializationUtils::integerToStream(numOfBlocks, stream);    
    // write char-block file to stream
    closeFile();
    fstream fstr(fname.c_str(), ios_base::in | ios_base::binary);
    // TODO if needed, for large files, copy by smaller chunks
    char* buff = new char[numOfBlocks];
    fstr.read(buff, numOfBlocks);
    stream.write(buff, numOfBlocks);
    fstr.close();
    delete [] buff;
    if (openFile()) state = STATE_OPENED;
    else state = STATE_ERROR;
    if (DEBUG) cout<<"writeToStream ended"<<endl;
}

void DiskCharArray::readFromStream(istream& stream) {
    if (DEBUG) cout<<"readFromStream, this.fname: "<<fname<<endl;
    int nb = SerializationUtils::integerFromStream<size_t>(stream);    
    char* buff = new char[nb];
    stream.read(buff, nb);    
    if (stream.good()) {        
        deleteFile();
        fstream fstr(fname.c_str(), ios_base::out | ios_base::binary);        
        fstr.write(buff, nb); fstr.close();        
        if (fstr.good() and openFile()) {
            numOfBlocks = nb;            
            state = STATE_OPENED;
            if (DEBUG) cout<<"writing success!"<<endl;
        }
        else {
            state = STATE_ERROR;
            numOfBlocks = 0;
            if (DEBUG) cout<<"writing error!"<<endl;
        }
    }
    delete [] buff;
    if (DEBUG) cout<<"readFromStream ended"<<endl;
}

DiskCharArray::~DiskCharArray() { 
    deleteFile();
}

