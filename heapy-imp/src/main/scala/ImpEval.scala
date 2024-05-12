// arithmetic expression
enum AExp {
    // variable in arithmetic expression
    case Var(s: String)

    // just an integer
    case Nat(n: Int)

    // addition
    case Add(add: AddExp)

    // pointing to a specific address in our heap
    case Pointer(addr: AExp)
}

// boolean expression
enum BExp {
    // constant true
    case True

    // constant false
    case False

    // negation
    case Neg(e: BExp)

    // comparison
    case Comp(c: CompExp)

    // conjunction
    case Conj(c: ConjExp)
}

// imp statement
enum ImpStmt {
    // assign (or load from heap)
    case Assign(a: AssignExp)

    // sequence
    case Seq(s: SeqExp)

    // conditional
    case Cond(c: CondExp)

    // no-op
    case Skip

    // while loop
    case While(w: WhileExp)

    case Break

    // write to heap
    case Write(w: WriteExp)
}

type ImpEvalContext = Map[String, Int]

// the default value for variable in arithmetic expression
val DefaultVal: Int = 0

extension (a: AExp) {
    // the interpreter / evaluator for arithmetic expression
    def aeval(sigma: ImpEvalContext): Int = {
        a match {
            case AExp.Add(add) => {
                var AddExp(a1: AExp, a2: AExp) = add
                a1.aeval(sigma) + a2.aeval(sigma)
            }
            case AExp.Nat(n) => n
            case AExp.Var(s) => {
                // we assigned a default "0" to every variable
                // that is not in the current context
                if sigma.contains(s) then {
                    sigma(s)
                } else {
                    DefaultVal
                }
            }
            case AExp.Pointer(addr) => {
                val address = addr.aeval(sigma)
                if heap.contains(address) then {
                    heap(address)
                } else {
                    DefaultVal
                }
            }
        }
    }
}

extension (b: BExp) {
    // the interpreter / evaluator for boolean expression
    def beval(sigma: ImpEvalContext): Boolean = {
        b match {
            case BExp.True   => true
            case BExp.False  => false
            case BExp.Neg(e) => !e.beval(sigma)
            // b1 && b2
            case BExp.Conj(c) => {
                val ConjExp(b1: BExp, b2: BExp) = c
                b1.beval(sigma) && b2.beval(sigma)
            }
            // if a1 <= a2
            case BExp.Comp(c) => {
                val CompExp(a1: AExp, a2: AExp) = c
                a1.aeval(sigma) <= a2.aeval(sigma)
            }
        }
    }
}

extension (i: ImpStmt) {
    // the interpreter / evaluator for imp
    def ieval(sigma: ImpEvalContext): (ImpEvalContext, Signal) = {
        i match {
            // e-skip
            case ImpStmt.Skip => (sigma, Signal.Continue)
            // e-break
            case ImpStmt.Break => (sigma, Signal.Break)
            // e-assign
            case ImpStmt.Assign(a) => {
                val x = a.x match
                    case AExp.Var(s: String) => s
                    case _ => assert(false, "expect `a.x` to be a variable")
                val v = a.v.aeval(sigma)
                // update the context
                (sigma + (x -> v), Signal.Continue)
            }
            // e-seq
            case ImpStmt.Seq(s) => {
                val (sigmaV1, signalV1) = s.i1.ieval(sigma)
                if signalV1.isBreak then (sigmaV1, signalV1)
                else s.i2.ieval(sigmaV1)
            }
            // e-cond
            case ImpStmt.Cond(c) => {
                if c.b.beval(sigma) then
                    // e-cond-true
                    c.i1.ieval(sigma)
                else
                    // e-cond-false
                    c.i2.ieval(sigma)
            }
            // e-while
            case ImpStmt.While(w) => {
                if w.b.beval(sigma) then
                    val (sigmaV1, signal) = w.i.ieval(sigma)
                    if signal.isBreak then (sigmaV1, Signal.Continue)
                    else i.ieval(sigmaV1)
                else (sigma, Signal.Continue)
            }
            // e-write
            case ImpStmt.Write(w) => {
                val address = w.addr.aeval(sigma)
                val value   = w.value.aeval(sigma)
                // update heap
                heap = heap + (address -> value)
                (sigma, Signal.Continue)
            }
        }
    }
}

// build the specific expression with the corresponding parameters
def build[T](args: Any*)(implicit ev: Buildable[T]): T = {
    // note that we must use `args*` here to expand the args
    // otherwise scala will automatically put `ArraySeq` on
    // top of the `args`, which has already been put by this function.
    ev.build(args*)
}

// a simple driver for evaluation
// the default context is an empty map
def eval(exp: ImpStmt): (ImpEvalContext, Signal) = {
    exp.ieval(Map())
}
