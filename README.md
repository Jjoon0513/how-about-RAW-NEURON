# how about RAW NEURON ( 쌩 뉴런은 어떤가요 )
외부 crate를 쓰지않고 오직 std로 구현한 인공신경망입니다.

- 외부 머신러닝 crate 미사용
- 순수 Rust 기반 구현
- Neuron / Layer / Network 구조 분리
- 활성화 함수 직접 구현 (ELU 등)
- 손실 함수 구현 (MSE)

## TODO

### 이미함
- [x] Forward propagation
- [x] Activation function (ReLU)
- [x] Loss function (MSE)

### 하는중
- [ ] Backpropagation

### 예정됨
- [ ] Training loop
- [ ] Optimizer (SGD?)
