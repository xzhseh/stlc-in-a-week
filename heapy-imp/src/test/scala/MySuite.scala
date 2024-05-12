class MySuite extends munit.FunSuite {
    import AExp.*
    import BExp.*
    import ImpStmt.*
    // x := 1;
    // y := 0;
    // while x <= 10 do {
    //     y := y + x;
    //     x := x + 1
    // }
    val i1 = Seq(
      SeqExp(
        // x := 1
        Assign(AssignExp(Var("x"), Nat(1))),
        Seq(
          SeqExp(
            // y := 0
            Assign(AssignExp(Var("y"), Nat(0))),
            While(
              WhileExp(
                // x <= 10
                Comp(CompExp(Var("x"), Nat(10))),
                Seq(
                  SeqExp(
                    // y := y + x
                    Assign(
                      AssignExp(Var("y"), Add(AddExp(Var("y"), Var("x"))))
                    ),
                    // x := x + 1
                    Assign(AssignExp(Var("x"), Add(AddExp(Var("x"), Nat(1)))))
                  )
                )
              )
            )
          )
        )
      )
    )

    // x := 1;
    // counter := 0;
    // while True do {
    //     x := x + x;
    //     counter := counter + 1;
    //     if counter == 10 then break
    // }
    val i2 = build[ImpStmt](
      // x := 1
      build[ImpStmt](build[AExp]("x"), build[AExp](1)),
      build[ImpStmt](
        // counter := 0
        build[ImpStmt](build[AExp]("counter"), build[AExp](0)),
        // while True
        build[ImpStmt](
          BExp.True,
          build[ImpStmt](
            // x := x + x
            build[ImpStmt](
              build[AExp]("x"),
              build[AExp](build[AExp]("x"), build[AExp]("x"))
            ),
            build[ImpStmt](
              // counter := counter + 1
              build[ImpStmt](
                build[AExp]("counter"),
                build[AExp](build[AExp]("counter"), build[AExp](1))
              ),
              // if 10 <= counter then break else skip
              build[ImpStmt](
                build[BExp](build[AExp](10), build[AExp]("counter")),
                ImpStmt.Break,
                ImpStmt.Skip
              )
            )
          )
        )
      )
    )

    // x := 0x114514;
    // h[x] = 1919810
    val i3 = build[ImpStmt](
      build[ImpStmt](build[AExp]("x"), build[AExp](0x114514)),
      build[ImpStmt]("write", build[AExp]("x"), build[AExp](1919810))
    )

    // y := h[0x114514];
    // y := y + 1;
    // h[0x114514] = y
    val i4 = build[ImpStmt](
      build[ImpStmt](build[AExp]("y"), build[AExp](build[AExp](0x114514))),
      build[ImpStmt](
        // y := y + 1
        build[ImpStmt](
          build[AExp]("y"),
          build[AExp](build[AExp]("y"), build[AExp](1))
        ),
        build[ImpStmt]("write", build[AExp](0x114514), build[AExp]("y"))
      )
    )

    test("aeval basic test") {
        val a1 = AExp.Nat(114514)
        val a2 = AExp.Nat(1919810)
        val a3 = AExp.Add(AddExp(a1, a2))
        assertEquals(a3.aeval(Map()), 114514 + 1919810)
    }

    test("beval basic test") {
        val a1 = AExp.Nat(114514)
        val a2 = AExp.Nat(1919810)
        val b  = BExp.Comp(CompExp(a1, a2))
        assertEquals(b.beval(Map()), true)
    }

    test("eval basic test 1") {
        val (sigma: ImpEvalContext, signal: Signal) = eval(i1)

        assert(!signal.isBreak, "expect `signal` to be continue")
        assertEquals(
          sigma.size,
          2,
          "expect there are exact 2 variables in `sigma`"
        )
        assertEquals(sigma("x"), 11, "the final state of `x` should be 11")
        assertEquals(sigma("y"), 55, "the final state of `y` should be 55")
    }

    test("eval basic test 2") {
        val (sigma: ImpEvalContext, signal: Signal) = eval(i2)

        assertEquals(sigma("counter"), 10)
        assertEquals(sigma("x"), 1024)
    }

    test("heap basic test") {
        val (sigma: ImpEvalContext, signal: Signal) = eval(i3)
        assertEquals(sigma("x"), 0x114514)
        assertEquals(heap(0x114514), 1919810)

        val (sigmaV1: ImpEvalContext, signalV1: Signal) = eval(i4)
        assertEquals(sigmaV1("y"), 1919811)
        assertEquals(heap(0x114514), 1919811)
    }
}
