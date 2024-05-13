trait Buildable[T] {
    def build(args: Any*): T
}

object Buildable {
    implicit object AExpBuildable extends Buildable[AExp] {
        def build(args: Any*): AExp = {
            validate("AExp", 2, args.size)
            args match {
                case Seq(v: AExp) if args.size == 1 => AExp.Pointer(v)
                case Seq(a1: AExp, a2: AExp) if args.size == 2 =>
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
            validate("BExp", 2, args.size)
            args match {
                case Seq(e: BExp) if args.size == 1 => BExp.Neg(e)
                case Seq(a1: AExp, a2: AExp) if args.size == 2 =>
                    BExp.Comp(CompExp(a1, a2))
                case Seq(b1: BExp, b2: BExp) if args.size == 2 =>
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
            validate("ImpStmt", 3, args.size)
            args match {
                case SSeq(x: AExp, v: AExp) if args.size == 2 =>
                    ImpStmt.Assign(AssignExp(x, v))
                case SSeq(_: String, x: AExp, v: AExp) if args.size == 3 =>
                    ImpStmt.Write(WriteExp(x, v))
                case SSeq(i1: ImpStmt, i2: ImpStmt) if args.size == 2 =>
                    ImpStmt.Seq(SeqExp(i1, i2))
                case SSeq(b: BExp, i1: ImpStmt, i2: ImpStmt)
                    if args.size == 3 =>
                    ImpStmt.Cond(CondExp(b, i1, i2))
                case SSeq(b: BExp, i: ImpStmt) if args.size == 2 =>
                    ImpStmt.While(WhileExp(b, i))
                case _ =>
                    assert(
                      false,
                      s"invalid build parameters for `ImpStmt`, args: $args"
                    )
            }
        }
    }
}

trait Into[S, T] {
    extension (e: S) def into: T
}

given Into[String, AExp] with {
    extension (e: String) def into: AExp = AExp.Var(e)
}

given Into[Int, AExp] with {
    extension (e: Int) def into: AExp = AExp.Nat(e)
}
