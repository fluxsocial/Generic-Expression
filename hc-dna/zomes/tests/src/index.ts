import { Orchestrator } from "@holochain/tryorama";
import { conductorConfig, installation } from "./common";

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
                "productId": 11
            }`
        },
    );
    console.log("**************")
    console.log("**************")
    console.log(entryHash);
    t.ok(entryHash);
});

orchestrator.run();
