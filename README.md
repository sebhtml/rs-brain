# Run the program

```bash
cargo run --release
```

# Run the tests

```bash
cargo test --release
```

# Mega_man

Mega_man.txt comes from Wikipedia .
Text is available under the Creative Commons Attribution-ShareAlike License 4.0

# Roadmap

- test argmax in output
- remove clipping
- add PredictWorkingMemory
- add a method assign in Tensor to avoid clone()
- do not use clone()
- separate Linear and activation
- learned Embedding
- Dropout
- move learning rate in dataset details
- add tape like in pytorch
- embeddings are learned
- shuffle examples in each epoch
- implement transformer
- bpe tokenizer
- add gelu

# Links

- [Training Hidden Units: The Generalized Delta Rule](https://web.stanford.edu/group/pdplab/originalpdphandbook/Chapter%205.pdf)
- [LINEAR](https://pytorch.org/docs/stable/generated/torch.nn.Linear.html)
- [Introducnn.Linear in PyTorch: Clearly Explainedtion](https://docs.kanaries.net/topics/Python/nn-linear)
- [Word Embeddings: Encoding Lexical Semantics](https://pytorch.org/tutorials/beginner/nlp/word_embeddings_tutorial.html)
- [A Simple Introduction to Broadcasting](https://medium.com/@hunter-j-phillips/)a-simple-introduction-to-broadcasting-db8e581368b3
- [Fast Multidimensional Matrix Multiplication on CPU from Scratch](https://siboehm.com/articles/22/Fast-MMM-on-CPU)
- [Mega man](https://en.wikipedia.org/wiki/Mega_Man)
- [Prof. Geoffrey Hinton - "Will digital intelligence replace biological intelligence?" Romanes Lecture](https://www.youtube.com/watch?v=N1TEjTeQeg0)
- [Attention Is All You Need](https://proceedings.neurips.cc/paper_files/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf)
- [GAUSSIAN ERROR LINEAR UNITS (GELUS)](https://arxiv.org/pdf/1606.08415.pdf)
- [What is Cross-Entropy Loss: LLMs Explained](https://www.chatgptguide.ai/2024/03/03/what-is-cross-entropy-loss-llms-explained/)
- [Deriving categorical cross entropy and softmax](https://shivammehta25.github.io/posts/deriving-categorical-cross-entropy-and-softmax/)