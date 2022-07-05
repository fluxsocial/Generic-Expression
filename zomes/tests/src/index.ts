import { Orchestrator } from "@holochain/tryorama";

let orchestrator = new Orchestrator();
require('./scenarios/public-expression')(orchestrator);
orchestrator.run();
