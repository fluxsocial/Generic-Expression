import { Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";

export const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const schemaValidation = path.join(__dirname, "../../../workdir/schema-validation.dna");

// Create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
export const installation: InstallAgentsHapps = [
    // agent 0
    [
        // happ 0
        [schemaValidation],
    ],
];

export const sleep = (ms) =>
    new Promise((resolve) => setTimeout(() => resolve(null), ms));
