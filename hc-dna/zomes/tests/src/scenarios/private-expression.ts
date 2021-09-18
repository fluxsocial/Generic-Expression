import { localConductorConfig, installation, sleep } from '../common';

module.exports = (orchestrator) => {
    orchestrator.registerScenario("Test public expression", async (s, t) => {
        const [alice, bob] = await s.players([localConductorConfig, localConductorConfig]);
    
        // install your happs into the coductors and destructuring the returned happ data using the same
        // array structure as you created in your installation array.
        const [[alice_happ]] = await alice.installAgentsHapps(installation);
        const [[bob_happ]] = await bob.installAgentsHapps(installation);

        await s.shareAllNodes([alice, bob]);

        const privateExpression = await alice_happ.cells[0].call(
            "schema_validation",
            "send_private_expression",
            {
                to: bob_happ.agent,
                expression: { 
                    data: `{
                        "productId": 1
                    }`,
                    author: "did://alice",
                    timestamp: new Date().toISOString(),
                    proof: {
                        signature: "sig",
                        key: "key"
                    },
                }
            },
        );
        console.log("Sent private experssion", privateExpression);
        t.ok(privateExpression);
    
        
    });
}