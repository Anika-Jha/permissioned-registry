# Submission — Permissioned Registry Smart Contract

---

This smart contract lets approved users store a single message on-chain while keeping everyone else’s data safe. Think of it like a shared notice board: only certain people can post, and once posted, the message cannot be changed.  
It demonstrates **how Web3 applications use blockchain to enforce permissions and immutability without a central server**.

---

1. The contract has an **owner** who controls who can write messages.  
2. Only **approved writers** can register messages.  
3. Each writer can store **one immutable message**.  
4. Writers **cannot edit anyone else’s message**.  
5. Anyone can **read all messages** and see the list of approved writers.  
6. Adding or removing writers is **restricted to the owner**.  
7. All writes are **permission-checked** to prevent unauthorized changes.  
8. Errors are **clear and explicit**, making debugging easy.  
9. State changes are **controlled and predictable**.  
10. The design prioritizes **clarity, security, and beginner-friendly readability**.
