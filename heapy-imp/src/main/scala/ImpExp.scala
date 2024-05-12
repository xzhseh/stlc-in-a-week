// add expression
case class AddExp(a1: AExp, a2: AExp)

// comparison
case class CompExp(a1: AExp, a2: AExp)

// conjunction
case class ConjExp(b1: BExp, b2: BExp)

// the signal for imp stmt
enum Signal {
    case Break, Continue
    def isBreak: Boolean = this == Signal.Break
}

// assign, i.e., x := v
case class AssignExp(x: AExp, v: AExp)

// sequence
case class SeqExp(i1: ImpStmt, i2: ImpStmt)

// condition
case class CondExp(b: BExp, i1: ImpStmt, i2: ImpStmt)

// while
case class WhileExp(b: BExp, i: ImpStmt)

// write, i.e., heap[addr] := value
case class WriteExp(addr: AExp, value: AExp)

// helper method for validation
def validate(exp: String, upperBound: Int, size: Int): Unit = {
    assert(
      size <= upperBound,
      s"expect the number of `args` used to build `$exp` is less or equal than $upperBound"
    )
}

// think of our memory-model for heapy-imp just as a mapping
// between the address and the corresponding value.
// it's worth noting that both the address and the value are
// arithmetic expression (i.e., AExp) in imp.
// plus, the initial value for every address is 0.
type Heap = Map[Int, Int]

// our *global* heap
// by making heap as a global value, we can actually
// "share" value between two or even more applications.
// note: the application is just a evaluable `ImpStmt`.
var heap: Heap = Map()
