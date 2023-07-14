# **g-zip**

g-zip aims to be an easy to use program to convert between binary information and dna-sequences.

## **What is g-zip used for?**
Every year humans produce unimaginably quantities of data. Most of that data is digital data.
For reference: in 2018 the humanity produced 64 zettabytes(10^21) of data and this number doubles every year. [source](https://rivery.io/blog/big-data-statistics-how-much-data-is-there-in-the-world/)

To store all of this data researchers work on different methods to improve current storage technology or invent new ways to store data. One of this new ways to store data is DNA.

DNA has a potentially storage density **100,000** times greater then that of traditional HDD's. [source](https://www.derstandard.de/story/2000077616743/bioinformatiker-dna-speicher-kann-man-nicht-hacken) (german article) To store binary data as DNA you have to convert binary to DNA. Here comes the use case of g-zip.

With g-zip you can convert every file on your computer to a DNA-sequence in a FASTA-file, use different algorithms to do that, add error-correcting codes, customize the FASTA-header and convert FASTA-files back to a binary file.

## **Examples**
Suppose you are a scientist working on a new method to store DNA containing binary information. For that you have to convert a file to DNA. But how do you convert a file to DNA? You could make a program that does the conversion for you yourself, but why should you waste your time making it from scratch when there is already is a program doing exactly that four you? You could use the gained time to polish your research even further before publishing.

In this case you could use g-zip (or at least I hope it could be someday) to convert your file to DNA and synthesize it to test your storage method.

## **Features**
### **1. DNA⇔Binary Conversion**
Currently there is one algorithm to convert between binary and genome. (There will hopefully be more added in the future!)
- easy-encode / easy-decode

    converts between binary and DNA by matching two bits to a nucleotide
    - 00 ⇔ A
    - 01 ⇔ T
    - 10 ⇔ C
    - 11 ⇔ G

### **2. Error correcting**
Again, currently there is one error correcting code available. (There will hopefully be more added in the future!)
- 7,4 Hamming-Code

    for further information on the 7,4 Hamming-Code see [here](https://en.wikipedia.org/wiki/Hamming(7,4))

### **3. FASTA-Header customization**


## **Installation**

## **Usage**
(use a couple screenshots and examples to make the reader familiar with g-zip)

## **Get in contact**

## **Roadmap (?)**
* More error-correcting algorithms
* more algorithms to convert between binary and dna
* FASTA-Header customization is not yet implemented.

## **Contributing**

