@main def hello(): Unit = {
    println("Congratulations, the program compiles.")

    var a                         = AExp.Var("a")
    var b                         = AExp.Nat(114514)
    var c                         = AExp.Add(AddExp(a, b))
    val context: Map[String, Int] = Map("a" -> 1919810)
    println(s"a + b = ${c.aeval(context)}, expected: ${114514 + 1919810}")

    val e = build[AExp](
      build[AExp]("a"),
      build[AExp](114514)
    )

    println(s"e.aeval = ${e.aeval(context)}, expected: ${114514 + 1919810}")
}
