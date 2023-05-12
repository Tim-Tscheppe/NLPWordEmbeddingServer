# NLPWordEmbeddingServer

### Introduction:

In NLP (Neural Language Processing), a lot of work still uses word embeddings to map from words to vectors. Typically, this process is done relatively inefficiently in Python, and today there exist many ML servers spending hundreds / thousands of megabytes of vectors in memory per web server process. The goal of this project is to implement an http "embedding" server using Rust that provides an API to perform the word embedding mapping in a faster and more memory efficient manner.

### Process:

1. Connect to API with Python
2. In Python, format word(s) into JSON
3. Post JSON to Rust Server
4. Read JSON
5. Perform word embeddings
6. Return JSON
7. Convert JSON back into vectors
8. Return outputs as a vector list

### To Build (On *Nix Machines)

git clone https://github.com/Tim-Tscheppe/NLPWordEmbeddingServer.git
cd NLPWordEmbeddingServer/server
cargo build
