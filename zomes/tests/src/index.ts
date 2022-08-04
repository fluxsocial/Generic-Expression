import test from "tape-promise/tape";
import { testPublicExpression } from "./scenarios/public-expression";


test("unsynced fetch", async (t) => {
    await testPublicExpression(t)
})