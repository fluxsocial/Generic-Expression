import { Scenario } from "@holochain/tryorama";
import { dnas, sleep } from "../common";

export async function testPublicExpression(t: any) {
    const scenario = new Scenario();
    const alice = await scenario.addPlayerWithHapp({dnas});

    // Create an expression
    let entryHash = await alice.cells[0].callZome({
        zome_name: "generic_expression",
        fn_name: "create_expression",
        payload: { 
            data: `{
                "productId": 1
            }`,
            author: "did://alice",
            timestamp: new Date().toISOString(),
            proof: {
                signature: "sig",
                key: "key"
            },
        },
    });
    console.log("Got entry hash: ")
    console.log(entryHash);
    t.ok(entryHash);

    // Create an invalid expression
    try {
        await alice.cells[0].callZome({
            zome_name: "generic_expression",
            fn_name: "create_expression",
            payload: { 
                data: `{
                    "productId": "1"
                }`,
                author: "did://alice",
                timestamp: new Date().toISOString(),
                proof: {
                    signature: "sig",
                    key: "key"
                },
            },
        });
    } catch(err) {
        console.log("Got expected error: ", err);
        t.ok(err);
    }

    sleep(10000);
    
    // Get expression by author
    var from_date = new Date();
    var dateOffset = 12 * 60 * 60 * 1000;
    from_date.setTime(from_date.getTime() - dateOffset); // 12 hours ago

    const expressions = await alice.cells[0].callZome({
        zome_name: "generic_expression",
        fn_name: "get_expression_by_author",
        payload: {
            author: "did://alice",
            from: from_date.toISOString(),
            until: new Date().toISOString()
        }
    });
    console.log("Got expressions by author: ", expressions);
    //@ts-ignore
    t.equal(expressions.length, 1);
    //@ts-ignore
    t.equal(expressions[0].data.productId, 1);
    //@ts-ignore
    t.equal(expressions[0].author, "did://alice");

    // Get experssion by its address
    const expression = await await alice.cells[0].callZome({
        zome_name: "generic_expression",
        fn_name: "get_expression_by_address",
        payload: entryHash,
    })
    console.log("Got expression by address: ", expression);
    //@ts-ignore
    t.equal(expression.data.productId, 1);
    //@ts-ignore
    t.equal(expression.author, "did://alice");

    await scenario.cleanUp();
}