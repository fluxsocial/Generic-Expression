import { Orchestrator } from "@holochain/tryorama";
import { conductorConfig, installation, sleep } from "./common";

const orchestrator = new Orchestrator();

orchestrator.registerScenario("Create expression", async (s, t) => {
    const [alice] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_common]] = await alice.installAgentsHapps(installation);

    let entryHash = await alice_common.cells[0].call(
        "schema_validation",
        "create_expression",
        { 
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
    );
    console.log("Got entry hash: ")
    console.log(entryHash);
    t.ok(entryHash);

    sleep(10000);
    
    // Get expression by author
    var from_date = new Date();
    var dateOffset = 12 * 60 * 60 * 1000;
    from_date.setTime(from_date.getTime() - dateOffset); // 12 hours

    const expressions = await alice_common.cells[0].call(
        "schema_validation",
        "get_expression_by_author",
        {
            author: "did://alice",
            from: from_date.toISOString(),
            until: new Date().toISOString()
        }
    );
    console.log("Got expressions: ", expressions);
    t.equal(expressions.length, 1);

});

orchestrator.run();
