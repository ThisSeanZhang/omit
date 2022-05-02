enum ValueType{
  NONE = 0, NUMBER = 1, ENUM = 2, STRING = 3, PAIR = 4,
}
type TypeMap = { [P in ValueType]: number; }

export function fromValue(data: number) : ValueType {
  switch (data) {
    case 1: return ValueType.NUMBER; break;
    case 2: return ValueType.ENUM; break;
    case 3: return ValueType.STRING; break;
    case 4: return ValueType.PAIR; break;
    default: return ValueType.NONE;
  }
}

export default ValueType;
