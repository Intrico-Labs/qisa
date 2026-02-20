# **Q-ISA v0.1 Formal Specification**

## **Execution Model**

### **1.1 Transactional Execution**

**A Q-ISA program is a Quantum Transaction**

#### **Properties:**

* Atomic (cannot be preempted mid-execution)  
* Linear (no branching in v0.1)  
* Deterministic instruction order  
* Probabilistic measurement outcome  
* Ends permanently on full measurement

### **1.2 Lifecycle**

1. Program loaded  
2. Header validated  
3. Logical qubits allocated  
4. Instruction stream executed sequentially  
5. Measurement results returned  
6. All qubits deallocated  
7. Program state destroyed

## **Binary File Layout**

| Header (64 Bytes) |
| :---- |
| Constant Pool  |
| Instruction Stream |
| Footer (Checksum) |

(All integers are little-endian)

## **Header Layout (64 Bytes)**

| Offset | Size | Field | Description |
| :---: | :---: | ----- | ----- |
| 0x00 | 4 | Magic Bytes | 0x4155544D (“AUTM”) |
| 0x04 | 2 | Version | 0x0001 (v0.1) |
| 0x06 | 2 | Flags | 0 for v0.1 |
| 0x08 | 4 | Logical Qubit Count | u32 |
| 0x0C | 4 | Classical Register Count | u32 |
| 0x10 | 8 | Instruction Count | u64 |
| 0x18 | 8 | Constant Pool Offset | u64 |
| 0x20 | 8 | Instruction Stream Offset | u64 |
| 0x28 | 8 | Constant Pool Size | u64 |
| 0x30 | 8 | Instruction Stream Size | u64 |
| 0x38 | 8 | Header Checksum | u64 |

Validation requires:

* Logical qubits \> 0  
* Instruction count \> 0  
* Stream offset valid  
* Program must end with QEND

## **Logical Qubit Model**

* Logical qubits indexed 0 … (N-1)  
* Must not exceed declared count  
* No dynamic creation beyond declared count  
* Reuse allowed only if explicitly freed

Qubit IDs are u32.

## **Classical Register Model**

* Classical registers indexed 0 … (M-1)  
* Only writable via QMEASURE  
* No classical arithmetic allowed in v0.1  
* Read-only after write

## **Constant Pool**

* Used for parameterized gates.  
* Constant pool index is u64.  
* Only f64 supported in v0.1.

## **Instruction Encoding**

All instructions begin with:  
`[u8 opcode]`  
`[operands...]`

## **Opcode Table (v0.1)**

### **Allocation**

| Opcode | Mnemonic |
| :---: | :---: |
| 0x01 | QINIT |

### **Single-Qubit Gates (No Parameter)**

| Opcode | Mnemonic |
| :---: | :---: |
| 0x10 | QH |
| 0x11 | QX |
| 0x12 | QY |
| 0x13 | QZ |

**Format**:  
`[u8 opcode]`  
`[u32 qubit_id]`

### **Single-Qubit Parameterized**

| Opcode | Mnemonic |
| :---: | :---: |
| 0x14 | QRX |
| 0x15 | QRY |
| 0x16 | QRZ |

**Format:**  
`[u8 opcode]`  
`[u32 qubit_id]`  
`[u64 const_index]`

### **Two-Qubit Gates**

| Opcode | Mnemonic |
| :---: | :---: |
| 0x20 | QCNOT |
| 0x21 | QSWAP |
| 0x22 | QCPHASE |

**Format:**  
`[u8 opcode]`  
`[u32 q1]`  
`[u32 q2]`

### **Synchronization**

| Opcode | Mnemonic |
| :---: | :---: |
| 0x30 | QBARRIER |
| 0x31 | QWAIT |

**Format:**  
`[u8 opcode]`  
`[u64 nanoseconds](* For QWAIT only)`

### **Measurement**

| Opcode | Mnemonic |
| :---: | :---: |
| 0x40 | QMEASURE |
| 0x41 | QMEASURE\_ALL |

**Format:**  
`[u8 opcode]`  
`[0x06]`  
`[u32 qubit_id] (* For QMEASURE only)`  
`[u32 classical_reg] (* For QMEASURE only)`

### **Termination**

| Opcode | Mnemonic |
| :---: | :---: |
| 0xF0 | QEND |

**Format:**  
`[u8 opcode]`

**\* Must be the final instruction.**

## **Validation Rules**

**AutmOS** will reject program if:

1. Missing QEND  
2. Instruction count mismatch  
3. Qubit index ≥ declared count  
4. Classical index ≥ declared count  
5. Invalid opcode  
6. Constant index out of bounds  
7. Instructions appear after QMEASURE\_ALL  
8. Header version mismatch

