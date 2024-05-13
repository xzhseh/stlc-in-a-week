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
      "seq",
      // x := 1
      build[ImpStmt]("assign", "x".into, 1.into),
      build[ImpStmt](
        "seq",
        // counter := 0
        build[ImpStmt]("assign", "counter".into, 0.into),
        // while True
        build[ImpStmt](
          "while",
          BExp.True,
          build[ImpStmt](
            "seq",
            // x := x + x
            build[ImpStmt](
              "assign",
              "x".into,
              build[AExp]("add", "x".into, "x".into)
            ),
            build[ImpStmt](
              "seq",
              // counter := counter + 1
              build[ImpStmt](
                "assign",
                "counter".into,
                build[AExp]("add", "counter".into, 1.into)
              ),
              // if 10 <= counter then break else skip
              build[ImpStmt](
                "cond",
                build[BExp]("comp", 10.into, "counter".into),
                ImpStmt.Break,
                ImpStmt.Skip
              )
            )
          )
        )
      )
    )

    // X := new(114514);
    // Y = X
    // *Y := 1919810
    val i3 = build[ImpStmt](
      "seq",
      build[ImpStmt]("alloc", build[AExp]("ref", "X"), 114514.into),
      build[ImpStmt](
        "seq",
        build[ImpStmt]("assign", build[AExp]("ref", "Y"), build[AExp]("ref", "X")),
        build[ImpStmt]("store", build[AExp]("deref", "Y"), 1919810.into)
      )
    )

    // x := *X
    // note: this will cause segmentation fault exception being thrown
    val i4 = build[ImpStmt](
      "assign",
      "x".into,
      build[AExp]("deref", "X")
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

      assertEquals(heap(sigma("X")), 1919810)
      assertEquals(heap(sigma("Y")), 1919810)
    }

    test("segfault basic test") {
      intercept[SegmentationFault] {
        val (_, _) = eval(i4)
      }
    }
}
