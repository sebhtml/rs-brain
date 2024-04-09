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

- one matmul to rule them all -> gemm
- use cublas https://docs.rs/cublas/latest/cublas/struct.API.html#method.gemm
- add capability for having N blocks side-by-side in a layer (required for multi-head attention)
- implement Dropout
- move learning rate in dataset details
- shuffle examples in each epoch
- implement transformer

- bpe tokenizer
- add gelu
- add tape to decouple compute from storage
- implement one-hot encoded tensor with sparsity
- centralize panic! calls
- implement tiled matrix multiplication

# General Links

- [A Simple Introduction to Broadcasting](https://medium.com/@hunter-j-phillips/)a-simple-introduction-to-broadcasting-db8e581368b3
- [Mega man](https://en.wikipedia.org/wiki/Mega_Man)
- [Prof. Geoffrey Hinton - "Will digital intelligence replace biological intelligence?" Romanes Lecture](https://www.youtube.com/watch?v=N1TEjTeQeg0)

# Performance Links

- [Fast Multidimensional Matrix Multiplication on CPU from Scratch](https://siboehm.com/articles/22/Fast-MMM-on-CPU)

# Mathematics Links

- [Training Hidden Units: The Generalized Delta Rule](https://web.stanford.edu/group/pdplab/originalpdphandbook/Chapter%205.pdf)
- [Attention Is All You Need](https://proceedings.neurips.cc/paper_files/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf)
- [GAUSSIAN ERROR LINEAR UNITS (GELUS)](https://arxiv.org/pdf/1606.08415.pdf)
- [What is Cross-Entropy Loss: LLMs Explained](https://www.chatgptguide.ai/2024/03/03/what-is-cross-entropy-loss-llms-explained/)
- [Deriving categorical cross entropy and softmax](https://shivammehta25.github.io/posts/deriving-categorical-cross-entropy-and-softmax/)

# PyTorch Links

- [LINEAR](https://pytorch.org/docs/stable/generated/torch.nn.Linear.html)
- [Introducnn.Linear in PyTorch: Clearly Explainedtion](https://docs.kanaries.net/topics/Python/nn-linear)
- [Word Embeddings: Encoding Lexical Semantics](https://pytorch.org/tutorials/beginner/nlp/word_embeddings_tutorial.html)

# CUDA links

- [Programming Tensor Cores in CUDA 9](https://developer.nvidia.com/blog/programming-tensor-cores-cuda-9/)
- [gemm](https://docs.rs/cublas/latest/cublas/struct.API.html#method.gemm)
- [arrayfire::CublasMathMode](https://arrayfire.org/arrayfire-rust/arrayfire/enum.CublasMathMode.html)
- [cuBLAS](https://docs.nvidia.com/cuda/cublas/)