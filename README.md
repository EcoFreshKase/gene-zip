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
- **easy-encode / easy-decode**

    converts between binary and DNA by matching two bits to a nucleotide
    - 00 ⇔ A
    - 01 ⇔ T
    - 10 ⇔ C
    - 11 ⇔ G

### **2. Error correcting**
Again, currently there is one error correcting code available. (There will hopefully be more added in the future!)
- **7,4 Hamming-Code**

    for further information on the 7,4 Hamming-Code see [here](https://en.wikipedia.org/wiki/Hamming(7,4))

### **3. FASTA-Header customization** (not implemented)
You can customize the header of the FASTA-file you want to save your DNA-sequence in. This allows you to store further information about the sequence you might need.


## **Installation**
An installer will be created in the future. For now you have to install g-zip yourself by forking the repository, compiling the code and changing the registry yourself.

## **Usage**
how do you use g-zip?
1. right-click the file you want to convert
2. select 'Start g-zip'
    
You should now see the following screen. (The paths on top will be different from file to file)

![g-zip after start](https://github.com/EcoFreshKase/g-zip/blob/README-creation/README%20images/g-zip%20after%20start.png)

On top there is "File-Path:". To the right of it you can see 2 paths. The top path of the two is the path of the file you will convert. Under that is the path the file be saved in. You can change it like you want.

In the middle of the screen there is a drop-down menu to select an error-correcting code.

Bellow the selection of error correcting codes is a button "config header" where you can, well, config the FASTA-header.

You can choose the algorithm to decode/encode your file on the right side. Firstly choose wether you want to decode or encode your file. After that you can choose from all available algorithms. After that you press the convert button and wait until your file is converted.


## **Roadmap**
g-zip is still under development. I still have many things I want to add. Here is a list of things I want to add in the future:
* an installer to, well, install the program
* possibility to customize the FASTA-Header
* improve the performance of easy-encode/easy-decode
* more algorithms to convert between binary and dna
* more error-correcting algorithms


## **Get in contact**
If you have any suggestions for the UI, have any problems, need more possibilities for DNA⇔Binary Conversion or have something on your mind you want to share feel free to open an issue describing whatever is on your mind or send me a message on discord (eco_fresh_kaese)

## **Contributing**
Feel free to work on an issue and make a pull request. I would be really happy if more people would work on my little project (～￣▽￣)～

A good first contribution would be to add a new error-correcting code or a new algorithm to convert between DNA and binary