trait Buildable[T] {
    def build(args: Any*): T
}

object Buildable {
    implicit object AExpBuildable extends Buildable[AExp] {
        def build(args: Any*): AExp = {
            validate("AExp", 3, args.size)
            args match {
                case Seq("ref", s: String) => AExp.Ref(s)
                case Seq("deref", s: String) => AExp.Deref(s)
                case Seq("add", a1: AExp, a2: AExp) =>
                    AExp.Add(AddExp(a1, a2))
                case _ =>
                    assert(
                      false,
                      s"invalid build parameters for `AExp`, args: $args"
                    )
            }
        }
    }

    implicit object BExpBuildable extends Buildable[BExp] {
        def build(args: Any*): BExp = {
            validate("BExp", 3, args.size)
            args match {
                case Seq("neg", e: BExp) => BExp.Neg(e)
                case Seq("comp", a1: AExp, a2: AExp) =>
                    BExp.Comp(CompExp(a1, a2))
                case Seq("conj", b1: BExp, b2: BExp) =>
                    BExp.Conj(ConjExp(b1, b2))
                case _ =>
                    assert(
                      false,
                      s"invalid build parameters for `BExp`, args: $args"
                    )
            }
        }
    }

    implicit object ImpStmtBuildable extends Buildable[ImpStmt] {
        def build(args: Any*): ImpStmt = {
            // to prevent naming conflict
            import scala.Seq as SSeq
            validate("ImpStmt", 4, args.size)
            args match {
                case SSeq("assign", x: AExp, v: AExp) =>
                    ImpStmt.Assign(AssignExp(x, v))
                case SSeq("seq", i1: ImpStmt, i2: ImpStmt) =>
                    ImpStmt.Seq(SeqExp(i1, i2))
                case SSeq("cond", b: BExp, i1: ImpStmt, i2: ImpStmt) =>
                    ImpStmt.Cond(CondExp(b, i1, i2))
                case SSeq("while", b: BExp, i: ImpStmt) =>
                    ImpStmt.While(WhileExp(b, i))
                case SSeq("alloc", ref: AExp, value: AExp) =>
                    ImpStmt.Alloc(AllocExp(ref, value))
                case SSeq("store", deref: AExp, value: AExp) =>
                    ImpStmt.Store(StoreExp(deref, value))
                case _ =>
                    assert(
                      false,
                      s"invalid build parameters for `ImpStmt`, args: $args"
                    )
            }
        }
    }
}

// syntax sugar for building `AExp.Var/Nat` from `String/Int`
trait Into[S, T] {
    extension (e: S) def into: T
}

given Into[String, AExp] with {
    extension (e: String) def into: AExp = AExp.Var(e)
}

given Into[Int, AExp] with {
    extension (e: Int) def into: AExp = AExp.Nat(e)
}
