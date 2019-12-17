# outside libs

Please place relevant lzt code here trying to preserve repo design:

```

./aux +-- lzt.hpp  // contains constructor, loader, query manager
      |
      +-- ./lzt +-- ...
                |
                +-- ...

```


## Signatures

### Constructor

```
Lzt();

```
I don't know if there are any options that need to be preset ...

### Loader/Builder

```
bool Lzt.make("/path/to/lzt/dict.lzt");  // assumes it does not exist and overwrites it



bool Lzt.read("/path/to/lzt/dict.lzt"); // check if dict exists , if not return false else return true

```
### Query Manager

Needs to accept vector of prefixes and return retrieved records

```
vector<char> get_fastq_records(vector<char>);

vector<char> get_fasta_records(vector<char>); // just a placeholder for now

/*
example:
   return vector<char> = "a, b, b, a, a, b, a, a, a, b, a, a, b, a, a, b, a, d, d";

   submit vector<char> = "a, a, b, \n, b, b, \n, ..."
   - '\n' prefix separator
*/

```
