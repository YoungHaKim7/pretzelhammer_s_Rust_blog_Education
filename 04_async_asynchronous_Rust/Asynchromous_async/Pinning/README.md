# Pinning and Select

- Of you want to create multiple futures as variables, and operate on them - you need to make sure that they will remain valid.
  - 변수로 여러 개의 futures선물을 생성하고 이를 운용하려면 해당 선물이 계속 유효한지 확인해야 합니다.
- Tokio's pin! macro makes this straightforward.
  - 토키오의 핀! 매크로는 이것을 간단하게 만듭니다
